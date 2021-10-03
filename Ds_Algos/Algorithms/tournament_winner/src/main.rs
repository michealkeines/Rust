use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Teams {
    A, B, C
}

fn tournament_winner(comp: &[Vec<Teams>], res: &[u32]) -> Option<Teams> {
    
    let mut temp = HashMap::new();
    for i in 0..res.len() {
        if res[i] == 0 {
            match temp.get(&comp[i][1]) {
                Some(value) => {
                    temp.insert(&comp[i][1], value + 1);
                },
                None => {
                    temp.insert(
                        &comp[i][1], 1
                    );
                }
            };
        } else {
            match temp.get(&comp[i][0]) {
                Some(value) => {
                    temp.insert(&comp[i][0], value + 1);
                },
                None => {
                    temp.insert(
                        &comp[i][0], 1
                    );
                }
            };
        }
    }

    let mut max: u32 = 0;
    let mut k: &Teams = &comp[0][0];
    for (key, value) in &temp {
        if value > &max {
            max = *value;
            k = key.clone();
        }
    }
    Some(k.clone())
}

fn main() {
    let competitors: Vec<Vec<Teams>> = vec![
        vec![Teams::A, Teams::B],
        vec![Teams::B, Teams::C],
        vec![Teams::C, Teams::A]
    ];

    let results: Vec<u32> = vec![0, 1, 0];

    let result: Option<Teams> = tournament_winner(&competitors, &results);

    match result {
        Some(team) => {
            match team {
                Teams::A => { println!("Team A Won the Tournament."); },   
                Teams::B => { println!("Team B Won the Tournament."); },
                Teams::C => { println!("Team C Won the Tournament."); }
            }
        },
        None => { println!("Error"); }
    };
}

// #[derive(Debug, Clone)]
// pub enum Values {
//     X(u32),
//     Y(Option<Teams>)
// }

// def test(current, maxval, key, res):
// 	if current > maxval:
// 		res[0] = current
// 		res[1] = key

// fn update_max(current: u32, val: Values, key: &Teams, res: &mut [Values]) {
//     let maxval = match val {
//         Values::X(x) => x,
//         _ => 0
//     };
//     let t = match key {
//         Teams::A => Teams::A,
//         Teams::B => Teams::B,
//         Teams::C => Teams::C,
//     };
 
//     if current > maxval {
//         res[0] = Values::X(current);
//         res[1] = Values::Y(Some(t));
//     }
// }

    // Games = [
    //     [TeamA, TeamB],
    //     [TeamB, TeamC],
    //     [TeamC, TeamA]
    // ]

    // Results = [ 0,1,1 ]



