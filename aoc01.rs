const INPUT: &'static str = r#"137503
60363
103031
141000
101650
76081
139069
63717
135021
66034
53912
83417
125978
73206
77497
108822
133339
113618
91973
88741
109942
96523
95973
56595
118638
63936
101635
149154
85522
140962
108196
105804
148464
68429
146808
82541
85581
117253
117900
83457
103354
123875
88412
108573
140651
103774
95291
91290
98690
87761
122907
91499
141746
127300
114866
75472
65369
50978
119756
144115
92483
146317
100770
124156
109933
138037
101126
58517
83653
135656
111483
82784
107459
106641
138030
53599
123886
74425
96919
65410
63823
148278
133753
106661
51147
120571
77900
131827
107882
149359
127565
67109
131547
114874
130493
94905
138654
58504
79591
133856"#;

fn main() {
  // split the input on newline
  // parse each line as a u32 mass
  // perform calculation for fuel
  // and sum them.
  let output = INPUT
    .split ("\n")
    .map (|mass| {
      // input is the mass of the module
      let mass = mass
        .trim ()
        .parse::<u32>()
        .unwrap ();
        
      // perform the calculation to find the
      // fuel (divide 3, truncate, subtract 2)
      let fuel = (((mass as f32) / 3.0) as i32) - 2;
      
      // track the total and new fuel
      // while the new fuel is positive, then
      // add to the total and update.
      let mut total = 0;
      let mut new = fuel;
      while new > 0 {
        total += new;
        new = (((new as f32) / 3.0) as i32) - 2;
      }
      
      // output the fuel amount for summation
      total
    })
    .sum::<i32>();
    
  // print output
  println!("{}", output)
}