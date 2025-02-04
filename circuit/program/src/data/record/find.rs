// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;

impl<A: Aleo> Record<A, Plaintext<A>> {
    /// Returns the entry from the given path.
    pub fn find(&self, path: &[Identifier<A>]) -> Result<Entry<A, Plaintext<A>>> {
        // If the path is of length one, check if the path is requesting the `owner`.
        if path.len() == 1 && path[0] == Identifier::from_str("owner")? {
            return Ok(self.owner.to_entry());
        }

        // Ensure the path is not empty.
        if let Some((first, rest)) = path.split_first() {
            // Retrieve the top-level entry.
            match self.data.get(first) {
                Some(entry) => match rest.is_empty() {
                    // If the remaining path is empty, return the top-level entry.
                    true => Ok(entry.clone()),
                    // Otherwise, recursively call `find` on the top-level entry.
                    false => entry.find(rest),
                },
                None => bail!("Record entry `{first}` not found."),
            }
        } else {
            bail!("Attempted to find record entry with an empty path.")
        }
    }
}
