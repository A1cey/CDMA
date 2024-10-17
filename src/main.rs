// 1. Bilden Sie drei Gruppen, dabei stellt jede Gruppe eine Station dar. Die
// Stationen sind gegeben durch:
// • A = (−1, −1, −1, + 1, + 1, −1, +1, + 1)
// • B = (−1, −1, + 1, −1, + 1, + 1, +1, −1)
// • C = (−1, + 1, −1, + 1, + 1, + 1, −1, −1)
// 2. Denken Sie sich eine Folge von 8bit (oder weniger) aus die Sie übertragen
// wollen und erstellen Sie daraus das Signal S = S1, S2, S3, S4, S5, S6, S7, S8
// Ihrer Station.
// 3. Geben Sie Ihr Signal an die Nachbargruppe weiter und erstellen Sie das
// resultierende Signal (SG1 + SG2) aus Ihrem Signal und dem der Nachbar-
// gruppe. Wiederholen Sie den Vorgang bis alle Signale in dem resultierenden Signal
// aufgenommen wurden (SG1 + SG2 + SG3).
// 4. Jede Gruppe sollte nun das resultierende Signal SG1 + SG2 + SG3 vorliegen haben.
// Stellen Sie fest welche bit-Folge die beiden andern Stationen
// gesendet haben, indem Sie das resultierende Signal SG1 + SG2 + SG3 skalar
// mit dem jeweiligen Stationscode multiplizieren (•).

// Station Code: -1 is send as 0
pub mod bits {
    #[derive(Copy, Clone, PartialEq)]
    pub enum Bit {
        ONE,
        ZERO,
    }

    impl Bit {
        pub fn convert_bit_str_to_bit_array<const LEN: usize>(bit_str: &str) -> [Bit; LEN] {
            let bit_str_len: usize = bit_str.len();

            if LEN != bit_str_len {
                panic!(
                    "The const BITS_LEN and the length of the parameter bit_str must be the same.\n
                    BITS_LEN: {LEN}, bit_str length: {bit_str_len}"
                )
            }

            let mut bit_array = [Bit::ZERO; LEN];

            for (idx, bit) in bit_str.chars().enumerate() {
                match bit {
                    '1' => bit_array[idx] = Bit::ONE,
                    '0' => bit_array[idx] = Bit::ZERO,
                    _ => {
                        panic!("Invalid bit: {bit} in bits: {bit_str}\nOnly 0 and 1 allowed.")
                    }
                };
            }

            bit_array
        }

        pub fn convert_bit_array_to_bit_str(bit_array: &[Bit]) -> String {
            let mut bit_str = String::new();

            for bit in bit_array {
                bit_str.push(match bit {
                    Bit::ONE => '1',
                    Bit::ZERO => '0',
                });
            }

            bit_str
        }
    }
}

pub mod client {
    use std::usize;

    use crate::bits::Bit;

    pub struct Client<const BITS_LEN: usize, const STATION_CODE_LEN: usize> {
        pub bits_to_send: [Bit; BITS_LEN],
        pub station_code: [i32; STATION_CODE_LEN],
        pub station_code_flipped: [i32; STATION_CODE_LEN],
    }

    impl<const BITS_LEN: usize, const STATION_CODE_LEN: usize> Client<BITS_LEN, STATION_CODE_LEN> {
        pub fn new(bits_to_send: &str, station_code: [i32; STATION_CODE_LEN]) -> Self {
            Self {
                bits_to_send: Bit::convert_bit_str_to_bit_array(bits_to_send),
                station_code,
                station_code_flipped: station_code.map(|val| val * -1),
            }
        }

        pub fn send_bits<'a>(&self) -> [[i32; STATION_CODE_LEN]; BITS_LEN] {
            let station_code_flipped: [i32; STATION_CODE_LEN] =
                self.station_code.map(|val| val * -1);

            let mut signal = [[0; STATION_CODE_LEN]; BITS_LEN];

            for (idx, bit) in self.bits_to_send.iter().enumerate() {
                signal[idx] = match bit {
                    Bit::ONE => self.station_code,
                    Bit::ZERO => station_code_flipped,
                };
            }

            Self::send(&signal);
            signal
        }

        fn send(bits: &[[i32; STATION_CODE_LEN]; BITS_LEN]) {
            bits.iter().for_each(|bit_arr| {
                bit_arr.iter().for_each(|bit| print!("{bit}, "));
                println!();
            });
            println!();
        }

        pub fn add_signals(
            signals: Vec<[[i32; STATION_CODE_LEN]; BITS_LEN]>,
        ) -> [[i32; STATION_CODE_LEN]; BITS_LEN] {
            let mut added_signals = [[0; STATION_CODE_LEN]; BITS_LEN];

            for signal in signals {
                for (station_code_idx, station_code) in signal.iter().enumerate() {
                    for (station_code_val_idx, station_code_val) in station_code.iter().enumerate()
                    {
                        added_signals[station_code_idx][station_code_val_idx] += station_code_val;
                    }
                }
            }

            Self::send(&added_signals);

            added_signals
        }

        pub fn get_signal_for_client(
            &self,
            added_signals: [[i32; STATION_CODE_LEN]; BITS_LEN],
        ) -> String {
            let client_station_codes =
                self.get_client_station_codes_from_added_signals(added_signals);

            let client_bits = self.convert_station_codes_to_bits(client_station_codes);

            let client_signal = Bit::convert_bit_array_to_bit_str(&client_bits);

            client_signal
        }

        fn dot_product<const LEN: usize>(
            nums: &[i32; LEN],
            station_code: [i32; STATION_CODE_LEN],
        ) -> i32 {
            let sum = nums
                .iter()
                .enumerate()
                .map(|(idx, num)| num * station_code[idx])
                .sum::<i32>();
            let num_of_nums: i32 = LEN.try_into().unwrap();
            sum / num_of_nums
        }

        fn get_client_station_codes_from_added_signals(
            &self,
            added_signals: [[i32; STATION_CODE_LEN]; BITS_LEN],
        ) -> [[i32; STATION_CODE_LEN]; BITS_LEN] {
            let mut station_codes = [[0; STATION_CODE_LEN]; BITS_LEN];

            for (idx, signal) in added_signals.iter().enumerate() {
                station_codes[idx] = match Self::dot_product(signal, self.station_code) {
                    -1 => self.station_code_flipped,
                    1 => self.station_code,
                    0 => {
                        println!("Client was not part of signal.");
                        [0; STATION_CODE_LEN]
                    }
                    _ => panic!("This is not possible."),
                };
            }

            Self::send(&station_codes);

            station_codes
        }

        fn convert_station_codes_to_bits(
            &self,
            station_codes: [[i32; STATION_CODE_LEN]; BITS_LEN],
        ) -> [Bit; BITS_LEN] {
            let zeroed_code = [0; STATION_CODE_LEN];

            let mut bits = [Bit::ZERO; BITS_LEN];

            for (idx, code) in station_codes.iter().enumerate() {
                if code.eq(self.station_code.as_ref()) {
                    bits[idx] = Bit::ONE;
                } else if code.eq(self.station_code_flipped.as_ref()) {
                    bits[idx] = Bit::ZERO;
                } else if code.eq(&zeroed_code) {
                    bits[idx] = Bit::ZERO;
                } else {
                    panic!("This is not possible.");
                }
            }

            bits
        }

        pub fn check_for_correct_transmission(&self, signal: &str) -> bool {
            signal == Bit::convert_bit_array_to_bit_str(self.bits_to_send.as_ref())
        }
    }
}

use client::Client;

fn main() {
    const BITS_LEN: usize = 8;
    const STATION_CODE_LEN: usize = 8;

    const CLIENT_A_BITS_TO_SEND: &str = "01001101";
    const CLIENT_B_BITS_TO_SEND: &str = "10011101";
    const CLIENT_C_BITS_TO_SEND: &str = "00010001";

    const CLIENT_A_STATION_CODE: [i32; STATION_CODE_LEN] = [-1, -1, -1, 1, 1, -1, 1, 1];
    const CLIENT_B_STATION_CODE: [i32; STATION_CODE_LEN] = [-1, -1, 1, -1, 1, 1, 1, -1];
    const CLIENT_C_STATION_CODE: [i32; STATION_CODE_LEN] = [-1, 1, -1, 1, 1, 1, -1, -1];

    let clients = vec![
        Client::<BITS_LEN, STATION_CODE_LEN>::new(CLIENT_A_BITS_TO_SEND, CLIENT_A_STATION_CODE),
        Client::<BITS_LEN, STATION_CODE_LEN>::new(CLIENT_B_BITS_TO_SEND, CLIENT_B_STATION_CODE),
        Client::<BITS_LEN, STATION_CODE_LEN>::new(CLIENT_C_BITS_TO_SEND, CLIENT_C_STATION_CODE),
    ];

    let mut signals: Vec<[[i32; STATION_CODE_LEN]; BITS_LEN]> = vec![];

    for (num, client) in clients.iter().enumerate() {
        // if num == 1 {
        //     continue;
        // }

        println!("Client {}:", num + 1);

        signals.push(client.send_bits());
    }

    println!("Added signal:");
    let added_signals = Client::<BITS_LEN, STATION_CODE_LEN>::add_signals(signals);

    let mut decoded_signals: Vec<String> = vec![];

    for (idx, client) in clients.iter().enumerate() {
        println!("Received signal for client {}: ", idx + 1);
        decoded_signals.push(client.get_signal_for_client(added_signals));
    }

    for (idx, client) in clients.iter().enumerate() {
        println!(
            "Checking correctness of transmission for client {}:",
            idx + 1
        );
        let is_correct = client.check_for_correct_transmission(decoded_signals[idx].as_str());

        println!(
            "Transmission was {}\n",
            if is_correct { "correct" } else { "incorrect" }
        )
    }
}
