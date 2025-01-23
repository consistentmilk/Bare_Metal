pub struct Solution;

enum CheckState {
    FloatSign, // Accepts: '+', | '-' | '.' | '0'..='9'
    FloatInit, // Accepts: '.' | '0'..='9'
    FloatNum,  // Accepts: '.' | 'e' | 'E' | '0'..='9'
    IntInit,   // Accepts: '0'..='9'
    IntNum,    // Accepts: 'e' | 'E' | '0'..='9'
    ExpSign,   // Accepts: '+' | '-' | '0'..='9'
    ExpInit,   // Accepts: '0'..='9'
    ExpNum,    // Accepts: '0'..='9'
}

impl CheckState {
    pub fn accept(&self, c: char) -> Result<Self, ()> {
        match (c, self) {
            ('+' | '-', Self::FloatSign) => Ok(Self::FloatInit),
            ('+' | '-', Self::ExpSign) => Ok(Self::ExpInit),
            ('.', Self::FloatSign | Self::FloatInit) => Ok(Self::IntInit),
            ('.', Self::FloatNum) => Ok(Self::IntNum),
            ('e' | 'E', Self::FloatNum | Self::IntNum) => Ok(Self::ExpSign),
            ('0'..='9', Self::FloatSign | Self::FloatInit | Self::FloatNum) => Ok(Self::FloatNum),
            ('0'..='9', Self::IntInit | Self::IntNum) => Ok(Self::IntNum),
            ('0'..='9', Self::ExpSign | Self::ExpInit | Self::ExpNum) => Ok(Self::ExpNum),
            _ => Err(()),
        }
    }

    pub fn is_valid_end_state(&self) -> bool {
        matches!(self, Self::FloatNum | Self::IntNum | Self::ExpNum)
    }
}

impl Solution {
    pub fn is_number(s: String) -> bool {
        let mut state = CheckState::FloatSign;

        for c in s.chars() {
            state = match state.accept(c) {
                Ok(new_state) => new_state,
                Err(()) => return false,
            };
        }
        state.is_valid_end_state()
    }
}
