use proc_macro_impl::PodDeSe;

#[derive(Debug, PodDeSe)]
pub struct UserData {
    age: u8,
    score: u16,
    m: u32,
    n: i8,
    nums: [u16; 5]
}

fn main() {
    let mut onchain_account_data: Vec<u8> = vec![
        18, // age
        45, 0, // score
        154, 0, 0, 0, // m
        45, // n
        34, 0, 54, 0, 222, 0, 43, 0, 99, 0 // nums
    ];

    unsafe {
        assert_eq!(
            vec![ 18 ],
            UserData::read_age(&onchain_account_data),
            "Invalid age field!"
        );

        assert_eq!(
            vec![ 45, 0 ],
            UserData::read_score(&onchain_account_data),
            "Invalid score field!"
        );

        assert_eq!(
            vec![ 154, 0, 0, 0 ],
            UserData::read_m(&onchain_account_data),
            "Invalid m field!"
        );

        assert_eq!(
            vec![ 45 ],
            UserData::read_n(&onchain_account_data),
            "Invalid n field!"
        );

        assert_eq!(
            vec![ 34, 0, 54, 0, 222, 0, 43, 0, 99, 0 ],
            UserData::read_nums(&onchain_account_data),
            "Invalid nums field!"
        );

        UserData::write_age(
            &mut onchain_account_data,
            &20u8.to_le_bytes()
        );
        assert_eq!(
            vec![ 20 ],
            UserData::read_age(&onchain_account_data)
        );

        UserData::write_score(
            &mut onchain_account_data,
            &233u16.to_le_bytes()
        );
        assert_eq!(
            vec![ 233, 0 ],
            UserData::read_score(&onchain_account_data)
        );

        UserData::write_m(
            &mut onchain_account_data,
            &199u32.to_le_bytes()
        );
        assert_eq!(
            vec![ 199, 0, 0, 0 ],
            UserData::read_m(&onchain_account_data)
        );

        UserData::write_n(
            &mut onchain_account_data,
            &1i8.to_le_bytes()
        );
        assert_eq!(
            vec![ 1 ],
            UserData::read_n(&onchain_account_data)
        );

        let new_nums: [u16; 5] = [ 11, 111, 22, 222, 33 ];
        let new_converted_nums: [u8; 10] = [
            new_nums[0] as u8, 0,
            new_nums[1] as u8, 0,
            new_nums[2] as u8, 0,
            new_nums[3] as u8, 0,
            new_nums[4] as u8, 0
        ];
        UserData::write_nums(
            &mut onchain_account_data,
            &new_converted_nums
        );
        assert_eq!(
            vec![ 11, 0, 111, 0, 22, 0, 222, 0, 33, 0 ],
            UserData::read_nums(&onchain_account_data)
        );
    };
}