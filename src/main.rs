
fn add_modulo2(x: u8, y: u8) -> u8 {
    let res = x + y;
    if res == 2 {0} else {res}
}


fn modulo_n_vectors(x: &Vec<u8>, y: &Vec<u8>) -> Vec<u8>{
    x.iter().zip(y.into_iter()).map(|(x_el, y_el)| add_modulo2(*x_el, *y_el) ).collect()
}


fn create_hex_binary(hex_value: u8) -> Vec<u8>{
    fn create_hex_binary_rec(hex_value: u8, counter: i32, acc: Vec<u8>) -> Vec<u8>{
        if counter < 0 {
            return acc;
        }
        let two_value = (2 as u32).pow(counter as u32) as u8;
        let new_counter = counter - 1;
        if hex_value >= two_value{
            let new_acc = functional_push_right(acc, 1);
            create_hex_binary_rec(hex_value - two_value, new_counter, new_acc)
        }else{
            let new_acc = functional_push_right(acc, 0);
            create_hex_binary_rec(hex_value, new_counter, new_acc)
        }
    }
    create_hex_binary_rec(hex_value, 3, vec![])
}

fn functional_push_right(vec: Vec<u8>, value: u8) -> Vec<u8> {
    // vec.into_iter().chain([value].into_iter()).collect()
    let mut vec_clone = vec.clone();
    vec_clone.push(value);
    vec_clone
}


fn binary_hex_to_value(binary_hex: &Vec<u8>) -> u8{
    8 * binary_hex.get(0).unwrap() + 4 * binary_hex.get(1).unwrap() + 2 * binary_hex.get(2).unwrap() + binary_hex.get(3).unwrap()
}

fn get_sbox() -> Vec<u8>{
    vec![0xE, 0x4, 0xD, 0x1, 0x2, 0xF, 0xB, 0x8, 0x3, 0xA, 0x6, 0xC, 0x5, 0x9, 0x0, 0x7]
}


// N = m = l = 4

fn generate_table(xprim: Vec<u8>, sbox: Vec<u8>) -> Vec<Vec<Vec<u8>>> {
    let res: Vec<Vec<Vec<u8>>> = (0..=15).into_iter().map(|x| {
        let x_binary = create_hex_binary(x);
        let xstar_as_binary = modulo_n_vectors(&x_binary, &xprim);
        let xstar = binary_hex_to_value(&xstar_as_binary);

        let y = sbox[x as usize];
        let y_binary = create_hex_binary(y);

        let ystar = sbox[xstar as usize];
        let ystar_binary = create_hex_binary(ystar);

        let yprim_binary =  modulo_n_vectors(&y_binary, &ystar_binary);
        vec![x_binary, xstar_as_binary, y_binary, ystar_binary, yprim_binary]

    }).collect();

    // vec![]
    res
}

fn count_occurences(values: Vec<u8>) -> Vec<u8>{
    let mut occurences: Vec<u8> = (0..=15).into_iter().map(|_i| 0).collect();
    for i in (0..=15).into_iter(){
        occurences[values[i] as usize] += 1;
    }
    occurences
    // vec![]
}

fn get_occurences_row(xprim: Vec<u8>) -> Vec<u8>{
    let sbox = get_sbox();
    let table = generate_table(xprim, sbox);

    let y_prim_column: Vec<Vec<u8>> = table.clone().into_iter().map(|row| {
        let res: &Vec<u8> = row.get(4).unwrap();
        res.clone()
    }).collect();
    let y_prim_column_values = y_prim_column.into_iter().map(|binary| {
        binary_hex_to_value(&binary)
    }).collect();

    let occurences_count = count_occurences(y_prim_column_values);
    occurences_count
}

fn main() {

    let rows: Vec<Vec<u8>> = (0..=15).into_iter().map(|x| {
        let xprim = create_hex_binary(x);
        let row =  get_occurences_row(xprim);
        row
    }).collect();
    // println!("{:?}", row);
    rows.into_iter().for_each(|row|{
        // println!("{:?}", row);
        row.into_iter().for_each(|count_value| {
            print!("{:2}", count_value)
        });
        println!()
    });
    println!("bye");
}
// w zadaniu 1 y = sbox(x) zrobic tabele z kolumnami x, x*, y, x*, y'
// w zadaniu 2 zrobic tabele zliczen 16x16 podobnie jak na poprzednich zajeciach