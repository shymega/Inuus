use std::io;
use std::path::PathBuf;
use std::process::Command;
use std::string;

#[derive(Debug, thiserror::Error)]
pub(crate) enum PassError {
    MalformedPassOutputError(#[source] string::FromUtf8Error),
    PassExecError(#[source] io::Error),
    GenericPassError,
}

pub(crate) struct PassSecretsProvider;
pub(crate) type PassResult<T, E = PassError> = anyhow::Result<T, E>;

impl PassSecretsProvider {
    pub(crate) fn find(&self, query: &String) -> PassResult<String> {
        let paths = self
            .get_paths()?
            .iter()
            .map(|p| p.to_string_lossy().into_owned())
            .filter(|p| p.contains(&self.query[..]))
            .collect();

        Ok(paths);
    }

    pub(crate) fn query(key: String) -> PassResult<String> {
        let output = Command::new("pass")
            .arg("show")
            .arg(key)
            .output()
            .map_err(PassError::PassExecError)?;

        let res = String::from_utf8(output.stdout)
            .map_err(PassError::MalformedPassOutputError)?;

        match res.as_str() {
            "" => Err(PassError::GenericPassError),
            _ => Ok(res),
        }
    }

    fn get_paths(&self) -> PassResult<Vec<PathBuf>> {
        let output = Command::new("pass")
            .arg("ls")
            .output()
            .map_err(PassError::PassExecError)?;

        let lines: Vec<String> = String::from_utf8(output.stdout)
            .map_err(PassError::MalformedPassOutputError)?
            .lines()
            .map(|c| String::from(c))
            .collect();

        self.tree_to_paths(Vec::from(&lines[1..]))
    }

    fn tree_to_paths(&self, lines: Vec<String>) -> PassResult<Vec<PathBuf>> {
        // List the tokens which indicate the start of indentation in the tree.
        let tokens = ["├── ", "└── "];

        // Translate the lines and tokens from strings to vectors of characters.
        let lines: Vec<_> = lines
            .iter()
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect();
        let tokens: Vec<_> = tokens
            .iter()
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect();

        // Initialize the vector to store the generated paths.
        let mut paths: Vec<PathBuf> = Vec::new();

        // Initialize the starting index of a found subtree to 0 and
        // set the name of the folder of this subtree
        let mut entry_idx = 0;

        for i in 1..lines.len() + 1 {
            // If a next token is found or the end of the tree has been reached,
            // compute the paths of entry_idx..i.
            if i == lines.len()
                || tokens.iter().any(|token| lines[i].starts_with(&token[..]))
            {
                // Get the top-level path of the found indentation
                let path_parent = PathBuf::from(
                    lines[entry_idx][4..].iter().collect::<String>(),
                );

                if entry_idx + 1 == i {
                    // If the newly found token is just one after the entry index,
                    // no indentation has occurred; add the current item to the path list.
                    paths.push(path_parent);
                } else {
                    // A new indentation token (or end of lines) has been reached,
                    // slice the lines from entry idx to here into a new subtree.
                    let indentation = lines[entry_idx + 1..i]
                        .iter()
                        .map(|ln| ln[4..].iter().collect::<String>())
                        .collect();

                    // Find the paths of the subtree.
                    // Make sure to prepend the current folder to the paths.
                    let children: Vec<PathBuf> = self.tree_to_paths(indentation)?
                        .iter()
                        .map(|path| path_parent.join(path))
                        .collect();

                    // Add the paths in this block to all the found paths.
                    paths.extend(children);
                }

                // Start looking for a subtree again from the current index.
                entry_idx = i;
            }
        }

        Ok(paths)
    }
}
