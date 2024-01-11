use aoc::prelude::*;

pub type ModuleID = u16;
pub type Modules = [FlipFlop; 65535];
static mut MODULES: [FlipFlop; 65535] = unsafe { std::mem::zeroed() };

pub fn parse(input: &str) -> ([ModuleID; 4], [ModuleID; 4], &'static Modules) {
    let mut crs = input.as_ptr();
    let mut broadcaster_outputs = [0; 4];
    let mut num_cycle_conjunctions = 0;
    let mut cycle_conjunctions = [0; 4];

    unsafe {
        for _ in 0..58 {
            match *crs {
                b'b' => {
                    crs.skip("broadcaster -> ".len());
                    parse_module_outputs_inplace::<4>(4, &mut broadcaster_outputs, &mut crs);
                }
                b'%' => {
                    let hash = hash(crs.add(1));
                    crs.skip("%aa -> ".len());

                    let num_module_outputs = count_flipflop_outputs(crs);

                    parse_module_outputs_inplace::<2>(
                        num_module_outputs,
                        &mut MODULES[hash as usize].outputs,
                        &mut crs,
                    );
                }
                b'&' => {
                    let hash = hash(crs.add(1));
                    crs.skip("%aa -> ".len());

                    let num_module_outputs = count_conjunction_outputs(crs);
                    if num_module_outputs == 5 {
                        cycle_conjunctions[num_cycle_conjunctions] = hash; // Only store cycle conjunctions
                        num_cycle_conjunctions += 1;

                        crs.skip("aa, bb, cc, dd, ee\n".len());
                    // Skip outputs
                    } else {
                        crs.skip("aa\n".len()); // Skip other conjunctions
                    }
                }
                _ => unreachable!(),
            }
        }

        (broadcaster_outputs, cycle_conjunctions, &MODULES)
    }
}

unsafe fn parse_module_outputs_inplace<const N: usize>(
    num_outputs: usize,
    outputs: &mut [ModuleID],
    crs: &mut *const u8,
) {
    outputs.iter_mut().take(num_outputs - 1).for_each(|output| {
        *output = hash(*crs);
        crs.skip("aa, ".len());
    });

    // Last output has no comma and ends with a newline
    outputs[num_outputs - 1] = hash(*crs);
    crs.skip("aa\n".len());
}

unsafe fn count_conjunction_outputs(crs: *const u8) -> usize {
    // #Outputs: {1:5, 4:4}
    if *crs.offset(2) == b'\n' {
        1
    } else {
        5
    }
}

unsafe fn count_flipflop_outputs(crs: *const u8) -> usize {
    // #Outputs: {1:16, 2:32}
    if *crs.offset(2) == b'\n' {
        1
    } else {
        2
    }
}

unsafe fn hash(name: *const u8) -> ModuleID {
    (name as *const ModuleID).read()
}

pub struct FlipFlop {
    pub outputs: [ModuleID; 2],
}
