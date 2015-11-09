// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

fn main() {
    let program = "+ + * - /";
    let mut accumulator = 0;

    for token in program {
        match token {
            '+' => accumulator += 1,
            '-' => accumulator -= 1,
            '*' => accumulator *= 2,
            '/' => accumulator /= 2,
            _ => { },
        }
    }

    println!("The program \"{}\" calculates the value {}", program, accumulator);
}
