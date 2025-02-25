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

mod utilities;
use utilities::*;

use console::network::prelude::*;
use snarkvm_synthesizer::Instruction;

#[test]
fn test_instruction_parse() {
    // Load the tests.
    let tests = load_tests::<_, LineParseTest>("./tests/parser/instruction", "./expectations/parser/instruction");
    // Run each test and compare it against its corresponding expectation.
    for test in &tests {
        // Run the parser on each of the test strings.
        let outputs = test
            .test_strings()
            .iter()
            .map(|test_string| convert_result(Instruction::<CurrentNetwork>::parse(test_string), test_string))
            .collect::<Vec<_>>();
        // Check against the expected output.
        test.check(&outputs).unwrap();
        // Save the output.
        test.save(&outputs).unwrap();
    }
}
