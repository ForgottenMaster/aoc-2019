///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
struct Reagent<'a> {
    name: &'a str,
    amount: u64
}

impl<'a> std::convert::From<&'a str> for Reagent<'a> {
    fn from(string: &'a str) -> Self {
        let mut splits = string
            .trim()
            .split(" ");
        let amount = splits
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let name = splits
            .next()
            .unwrap();
        Self { name, amount }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
struct Reaction<'a> {
    inputs: Box<[Reagent<'a>]>,
    output: Reagent<'a>
}

impl<'a> std::convert::From<&'a str> for Reaction<'a> {
    fn from(string: &'a str) -> Self {
        let mut splits = string
            .trim()
            .split(" => ");
        let inputs = splits
            .next()
            .unwrap()
            .trim()
            .split(",")
            .map(|reagent| {
                reagent.trim().into()
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let output = splits
            .next()
            .unwrap()
            .trim()
            .into();
        Self { inputs, output }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
struct Nanofactory<'a> {
    contents: std::collections::HashMap<&'a str, u64>,
    reactions: std::collections::HashMap<&'a str, Reaction<'a>>,
    ore_required: u64,
    ore_production: bool
}

impl<'a> Nanofactory<'a> {
    fn set_fixed_ore_pool(&mut self, amount: u64) {
        self.ore_production = false;
        *self.contents.entry("ORE").or_insert(0) = amount;
    }

    fn fabricate(&mut self, mut amount: u64, reagent: &'a str) -> bool {
        // if there's any in the factory, then reduce the amount required
        // by whatever is left.
        if let Some(in_factory) = self.contents.get_mut(reagent) {
            let amount_used = std::cmp::min(amount, *in_factory);
            amount -= amount_used;
        }
        
        // if there's nothing to be produced, then we are successful.
        if amount == 0 {
            true
        } else {
            // ensure that the reagent is in the mapping of contents of the factory.
            self.contents.entry(reagent).or_insert(0);
            
            // if the reagent is "ORE" just magic some up, otherwise we need
            // to look at the reaction to produce it.
            if reagent == "ORE" {
                if self.ore_production {
                    *self.contents.get_mut(reagent).unwrap() += amount;
                    self.ore_required += amount;
                    true
                } else {
                    false // can't produce ore, but need more, so failed
                }
            } else {
                // need to copy the input reagents into a vector as we can't hold onto
                // an immutable borrow since we need the mutable borrow for the fabrication
                // of said inputs. Therefore we can't just iterate over the reagent list
                // directly from the mapping.
                let (inputs, times) = {
                    let reaction = self.reactions.get(reagent).unwrap();
                    let mut inputs = Vec::with_capacity(reaction.inputs.len());
                    for input in reaction.inputs.iter() {
                        inputs.push((input.amount, input.name));
                    }
                    let times = (amount as f64 / reaction.output.amount as f64).ceil() as u64;
                    (inputs, times)
                };
                
                // fabricate the required amount of each of the inputs, and then immediately consume the
                // correct amount from the factory contents.
                let mut all_inputs_fabricated = true;
                for (amount, reagent) in inputs {
                    if self.fabricate(amount * times, reagent) {
                        *self.contents.get_mut(reagent).unwrap() -= amount * times;
                    } else {
                        all_inputs_fabricated = false;
                        break;
                    }
                }
                
                // if all inputs were successfully fabricated, add the output in.
                if all_inputs_fabricated {
                    *self.contents.get_mut(reagent).unwrap() += self.reactions.get(reagent).unwrap().output.amount * times;
                    true
                } else {
                    false
                }
            }
        }
    }
}

impl<'a> std::convert::From<&'a str> for Nanofactory<'a> {
    fn from(string: &'a str) -> Self {
        let contents = std::collections::HashMap::new();
        let reactions = string
            .trim()
            .lines()
            .map(|line| {
                let reaction: Reaction<'a> = line.into();
                (reaction.output.name, reaction)
            })
            .collect();
        let ore_required = 0;
        let ore_production = true;
        Self { contents, reactions, ore_required, ore_production }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn find_middle(lower_bound: u64, upper_bound: u64) -> u64 {
    if lower_bound >= upper_bound {
        lower_bound
    } else {
        lower_bound + (upper_bound - lower_bound) / 2
    }
}

fn main() {
    const INPUT: &'static str = r#"
        3 LMPDB, 11 CBTKP => 7 PZDPS
        5 CBKW, 4 CXBH => 9 KXNDF
        1 LVDN, 4 HGDHV => 1 PCXS
        11 PCXS => 2 XTBRS
        5 RVSF => 7 TDCH
        1 CXBH, 6 PXVN => 8 GQXV
        3 DBCB, 3 QLNK => 4 CTFCD
        7 PZDPS, 18 HGDHV, 9 TBKM => 4 JHVL
        10 QGSV, 1 DBCB, 7 LTHFX => 3 BLRSQ
        12 CBTKP, 7 SPBF => 5 KSQL
        1 QXHDQ, 5 MQKH, 10 XRCB, 30 SQWHX, 2 PQZVD, 30 TFST, 39 JPFC, 1 FDGS, 17 LVDN => 1 FUEL
        2 TBKM => 8 PFHKT
        13 CBTKP => 5 QLNK
        12 TVRDM, 6 QGSV, 16 LMPDB => 4 PQZVD
        7 TDCH, 17 PXVN, 4 ZLKZ => 6 XRCB
        1 QBJQ, 26 CBKW => 4 RVSF
        24 KXNDF, 3 BLRSQ => 9 GSHKQ
        12 BLRSQ, 3 HGDHV => 9 RQNGQ
        2 RFBK, 2 WHWS => 8 CBKW
        1 WHWS => 7 LTHFX
        13 CKQLD, 10 ZLKZ, 2 GQXV => 8 TVHC
        1 DBCB => 2 JZXKW
        8 SPBF => 7 CXBH
        11 LTHFX, 1 PTGLG, 10 NCQTM => 6 SQWHX
        16 PFHKT => 3 HGDHV
        3 LVDN, 5 PZDPS, 1 SPBF => 9 CQBCL
        19 BLRSQ, 1 BLQRD, 5 GSHKQ, 2 LVDN, 3 LMPDB, 5 KTJR => 1 QXHDQ
        1 RFBK, 1 JPFC => 7 PXVN
        110 ORE => 3 MQKH
        1 FPBRB, 7 MQKH => 7 SDJBT
        128 ORE => 7 FPBRB
        3 WRWGP => 2 RFBK
        1 PFHKT, 4 SPBF => 7 JPFC
        14 LTHFX, 2 JZXKW, 2 BLRSQ, 2 MHVJP, 6 RQNGQ, 1 CQBCL, 8 TDCH, 2 NJTR => 2 FDGS
        4 SDJBT, 2 LMPDB => 8 PLGS
        1 RFBK, 1 TBKM => 6 CBTKP
        17 LVDN, 2 CBTKP => 4 QGSV
        7 WRWGP => 9 LMPDB
        3 CKQLD => 6 WHWS
        14 CBTKP, 9 XTBRS, 9 GSHKQ, 12 GQXV, 20 LTHFX, 1 RQNGQ, 1 KTJR, 3 BLRSQ => 7 TFST
        1 QPCQ => 5 BLQRD
        6 QGSV, 1 HGDHV, 1 JPFC => 1 NJTR
        1 HGDHV, 7 JHVL, 5 PZDPS => 9 MGRT
        1 KSQL => 5 QBJQ
        2 QLNK => 2 CKQLD
        13 JZXKW, 14 XTBRS => 3 PTGLG
        1 BNPXT, 2 PLGS => 7 DBCB
        1 RFBK, 9 CTFCD => 1 MHVJP
        1 NJTR, 1 TVHC, 2 PCXS => 1 KTJR
        2 WRWGP => 6 TBKM
        12 QLNK, 1 NJTR => 3 NCQTM
        13 ZHCKP, 2 DBCB, 5 PXVN => 9 QPCQ
        125 ORE => 3 WRWGP
        6 CBTKP, 9 TBKM => 9 SPBF
        1 GQXV => 6 ZHCKP
        1 MGRT => 8 BNPXT
        2 SPBF => 4 ZLKZ
        9 TVHC, 5 KXNDF, 3 QPCQ => 3 TVRDM
        1 PLGS, 7 TBKM => 8 LVDN
    "#;
    
    // part 1
    let ore_required_per_fuel = {
        let mut nanofactory: Nanofactory = INPUT.into();
        nanofactory.fabricate(1, "FUEL");
        println!("Ore Required: {}", nanofactory.ore_required);
        nanofactory.ore_required
    };
    
    // part 2
    {
        const ORE_CAP: u64 = 1_000_000_000_000;
        let mut nanofactory: Nanofactory = INPUT.into();
        nanofactory.set_fixed_ore_pool(ORE_CAP);
        let (mut lower_bound, mut upper_bound) = (ORE_CAP / ore_required_per_fuel, ORE_CAP);
        while lower_bound < upper_bound {
            let halfway = find_middle(lower_bound, upper_bound);
            if nanofactory.clone().fabricate(halfway, "FUEL") {
                lower_bound = halfway + 1;
            } else {
                upper_bound = halfway - 1;
            }
        }
        let final_total = lower_bound - 1; // we increment the lower bound after the final success before breaking out of the loop.
        println!("Maximum Amount Of FUEL From {} ORE = {}", ORE_CAP, final_total);
    }
}
