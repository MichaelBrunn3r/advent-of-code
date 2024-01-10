use aoc::{ConstVec, Cursor};
static mut WF_HASH_TO_ID: [u16; 65536] = gen_wf_hash_to_id();
static mut WORKFLOWS: [Workflow; 1650] = unsafe { std::mem::zeroed() };
static mut MAX_WF_ID: u16 = 0;
static mut RULES: ConstVec<Rule, 1599> = ConstVec::new(Rule::new_reject_all());
const NUM_PARTS: usize = 200;
static mut PARTS: [Part; NUM_PARTS] = unsafe { std::mem::zeroed() };
pub const WF_IN_ID: usize = 0;

pub fn parse_workflows(crs: &mut Cursor<u8>) -> (&'static [Workflow; 1650], &'static [Rule]) {
    unsafe {
        RULES.clear();
        MAX_WF_ID = 1; // "in" is always at index 0

        while crs[0] != b'\n' {
            let mut wf_id_start = *crs;
            let wf_name_len = if crs[2] == b'{' { 2 } else { 3 };
            crs.skip(wf_name_len);
            crs.skip("{".len());

            let wf_rules_start = RULES.len;

            RULES.push(parse_rule(crs));
            loop {
                crs.skip(",".len());
                if crs[1] == b'<' || crs[1] == b'>' {
                    RULES.push(parse_rule(crs));
                } else {
                    RULES.push(parse_terminal_rule(crs));
                    break;
                }
            }
            crs.skip("}\n".len());

            let wf_id = parse_wf_name_to_id(&mut wf_id_start, wf_name_len);
            WORKFLOWS[wf_id as usize] = (wf_rules_start, RULES.len);
        }

        (&WORKFLOWS, &RULES)
    }
}

pub fn parse_parts(crs: &mut Cursor<u8>) -> &'static [Part] {
    unsafe {
        for part in PARTS.iter_mut() {
            crs.skip("{".len());

            let mut xmas = [0u16, 0, 0, 0];
            for rating in &mut xmas {
                crs.skip("x=".len());
                let num_rating_digits = get_num_rating_digits(crs);
                *rating = crs.parse_uint_n_digits(num_rating_digits);
                crs.skip(",".len()); // Also skips terminating '}'
            }
            crs.skip("\n".len());

            part.0 = xmas;
        }

        &PARTS
    }
}

fn parse_rule(crs: &mut Cursor<u8>) -> Rule {
    let rating = Rating::from_ascii_char(crs.take());
    let condition_type = crs.take();

    let num_condition_digits = get_on_met_separator_offset(crs);
    let condition_value: u32 = crs.parse_uint_n_digits(num_condition_digits);
    crs.skip(":".len());

    let condition = Condition::from_ascii_char(condition_type, condition_value);

    let (on_met, on_met_id) = match crs[0] {
        b'A' => {
            crs.skip("A".len());
            (OnMet::Accept, u16::MAX)
        }
        b'R' => {
            crs.skip("R".len());
            (OnMet::Reject, u16::MAX)
        }
        _ => {
            let wf_name_len = if crs[2] == b',' { 2 } else { 3 };
            let on_met_id = parse_wf_name_to_id(crs, wf_name_len);
            (OnMet::Continue, on_met_id)
        }
    };

    Rule {
        rating,
        condition,
        on_met,
        on_met_id,
    }
}

fn parse_terminal_rule(crs: &mut Cursor<u8>) -> Rule {
    match crs[0] {
        b'R' => {
            crs.skip("R".len());
            Rule::new_reject_all()
        }
        b'A' => {
            crs.skip("A".len());
            Rule::new_accept_all()
        }
        _ => {
            let wf_name_len = if crs[2] == b'}' { 2 } else { 3 };
            let on_met_wf_id = parse_wf_name_to_id(crs, wf_name_len);
            Rule::new_continue_all(on_met_wf_id)
        }
    }
}

/** Get the length of the name of the current workflow. Starts at the start of the line and is terminated by '{' */
fn get_wf_name_length(crs: &mut Cursor<u8>) -> usize {
    // Name lengths: [3: 310, 2: 229]
    if !crs[3].is_ascii_lowercase() {
        3
    } else {
        2
    }
}

fn parse_wf_name_to_id(crs: &mut Cursor<u8>, len: usize) -> u16 {
    unsafe {
        let hash = hash_wf_name(crs, len);

        if WF_HASH_TO_ID[hash as usize] == u16::MAX {
            let id = MAX_WF_ID;
            WF_HASH_TO_ID[hash as usize] = id;
            MAX_WF_ID += 1;
            id
        } else {
            WF_HASH_TO_ID[hash as usize]
        }
    }
}

fn hash_wf_name(crs: &mut Cursor<u8>, len: usize) -> u16 {
    let mut hash = 0;
    for _ in 0..len {
        hash <<= 5;
        hash |= (crs.take() - b'a') as u16;
    }
    hash
}

const fn hash_wf_name_const(bytes: &[u8]) -> u16 {
    let mut hash = 0;
    let mut i = 0;
    while i < bytes.len() {
        hash <<= 5;
        hash |= (bytes[i] - b'a') as u16;
        i += 1;
    }
    hash
}

/** Get the offset of '}' terminating the list of rules of the current workflow */
fn get_rules_list_terminator_offset(data: &mut Cursor<u8>) -> usize {
    // Name lengths: [2: 229, 3: 310]
    if data[3] == b'}' {
        3
    } else {
        2
    }
}

/** Get offset of ':' separating the current rules condition and its 'on-met' part */
fn get_on_met_separator_offset(crs: &mut Cursor<u8>) -> usize {
    // Condition digits: [2: 19, 3: 313, 4: 728]
    if crs[4] == b':' {
        4
    } else if crs[3] == b':' {
        3
    } else {
        2
    }
}

fn get_num_rating_digits(crs: &Cursor<u8>) -> usize {
    if crs[3].is_ascii_digit() {
        4
    } else if crs[2].is_ascii_digit() {
        3
    } else if crs[1].is_ascii_digit() {
        2
    } else {
        1
    }
}

pub type Workflow = (u16, u16);

#[derive(Debug, Clone, Copy)]
pub struct Rule {
    pub rating: Rating,
    pub condition: Condition,
    pub on_met: OnMet,
    pub on_met_id: u16,
}

impl Rule {
    pub fn is_met(&self, part: &Part) -> bool {
        self.condition.is_met(part.0[self.rating as usize & 0b11])
    }

    pub const fn new_accept_all() -> Self {
        Rule {
            rating: Rating::Any,
            condition: Condition::LessThan(4001),
            on_met: OnMet::Accept,
            on_met_id: u16::MAX,
        }
    }

    pub const fn new_reject_all() -> Self {
        Rule {
            rating: Rating::Any,
            condition: Condition::LessThan(4001),
            on_met: OnMet::Reject,
            on_met_id: u16::MAX,
        }
    }

    pub const fn new_continue_all(on_met_wf_id: u16) -> Self {
        Rule {
            rating: Rating::Any,
            condition: Condition::LessThan(4001),
            on_met: OnMet::Continue,
            on_met_id: on_met_wf_id,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum OnMet {
    Accept,
    Reject,
    Continue,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Rating {
    X = 0,
    M = 1,
    A = 2,
    S = 3,
    Any = 4,
}

impl Rating {
    #[inline(always)]
    pub fn from_ascii_char(c: u8) -> Self {
        const LUT: [Rating; 121] = Rating::_create_lut();
        LUT[c as usize]
    }

    const fn _create_lut() -> [Rating; 121] {
        let mut lut = [Rating::Any; 121];
        lut[b'x' as usize] = Rating::X;
        lut[b'm' as usize] = Rating::M;
        lut[b'a' as usize] = Rating::A;
        lut[b's' as usize] = Rating::S;
        lut
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Condition {
    LessThan(u16) = b'<',
    GreaterThan(u16) = b'>',
}

impl Condition {
    #[inline(always)]
    pub fn from_ascii_char(c: u8, value: u32) -> Self {
        unsafe { std::mem::transmute(c as u32 | value << 16) }
    }

    pub fn is_met(&self, value: u16) -> bool {
        match self {
            Self::LessThan(n) => value < *n,
            Self::GreaterThan(n) => value > *n,
        }
    }
}

#[derive(Debug)]
pub struct Part(pub [u16; 4]);

const fn gen_wf_hash_to_id() -> [u16; 65536] {
    let mut hash_to_id = [u16::MAX; 65536];
    hash_to_id[hash_wf_name_const(b"in") as usize] = WF_IN_ID as u16; // "in" is always at index 0
    hash_to_id
}
