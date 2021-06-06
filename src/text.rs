use phf::{phf_map, Map};
use rand::{rngs::StdRng, Rng};

use super::rng::rng_for_maze;

pub fn generate_name(seed: u64, position: (i64, i64), name: &mut String) {
	let mut rng: StdRng = rng_for_maze(seed, position);
	let desired_length = rng.gen_range(4..10);
	let original_length = name.len();

	let mut length = 0;
	while length < desired_length {
		if length < 2 {
			name.push_str(HD[rng.gen_range(0..HD.len())]);
			length += 2;
		} else {
			let d = &name[original_length + length - 2..];
			if let Some(ch) = DI
				.get(d)
				.map(|dh| {
					if length + 1 == desired_length {
						dh.end
					} else {
						dh.middle
					}
				})
				.filter(|ch| !ch.is_empty())
			{
				name.push(ch[rng.gen_range(0..ch.len())]);
				length += 1;
			} else {
				length -= 3.min(length);
				name.truncate(original_length + length);
			}
		}
	}
}

struct DiHolder {
	middle: &'static [char],
	end: &'static [char],
}
const HD: [&str; 163] = [
	"TH", "OF", "AN", "IN", "TO", "CO", "BE", "HE", "RE", "HA", "WA", "FO", "WH", "MA", "WI", "ON",
	"HI", "PR", "ST", "NO", "IS", "IT", "SE", "WE", "AS", "CA", "DE", "SO", "MO", "SH", "DI", "AL",
	"AR", "LI", "WO", "FR", "PA", "ME", "AT", "SU", "BU", "SA", "FI", "NE", "CH", "PO", "HO", "DO",
	"OR", "UN", "LO", "EX", "BY", "FA", "LA", "LE", "PE", "MI", "SI", "YO", "TR", "BA", "GO", "BO",
	"GR", "TE", "EN", "OU", "RA", "AC", "FE", "PL", "CL", "SP", "BR", "EV", "TA", "DA", "AB", "TI",
	"RO", "MU", "EA", "NA", "SC", "AD", "GE", "YE", "AF", "AG", "UP", "AP", "DR", "US", "PU", "CE",
	"IF", "RI", "VI", "IM", "AM", "KN", "OP", "CR", "OT", "JU", "QU", "TW", "GA", "VA", "VE", "PI",
	"GI", "BI", "FL", "BL", "EL", "JO", "FU", "HU", "CU", "RU", "OV", "MY", "OB", "KE", "EF", "PH",
	"CI", "KI", "NI", "SL", "EM", "SM", "VO", "MR", "WR", "ES", "DU", "TU", "AU", "NU", "GU", "OW",
	"SY", "JA", "OC", "EC", "ED", "ID", "JE", "AI", "EI", "SK", "OL", "GL", "EQ", "LU", "AV", "SW",
	"AW", "EY", "TY",
];
const DI: Map<&'static str, DiHolder> = phf_map! {
	"TH" => DiHolder {
		middle: &[
			'E',
			'A',
			'I',
			'O',
			'R'
		],
		end: &[
			'E',
			'O'
		]
	},
	"AN" => DiHolder {
		middle: &[
			'D',
			'T',
			'Y',
			'C',
			'S',
			'G',
			'N',
			'I',
			'O',
			'E',
			'A',
			'K'
		],
		end: &[
			'D',
			'T',
			'Y',
			'S',
			'G',
			'O',
			'E',
			'A',
			'K'
		]
	},
	"IN" => DiHolder {
		middle: &[
			'G',
			'T',
			'E',
			'D',
			'S',
			'C',
			'A',
			'I',
			'K',
			'V',
			'U',
			'N',
			'F'
		],
		end: &[
			'G',
			'T',
			'E',
			'D',
			'S',
			'A',
			'K'
		]
	},
	"IO" => DiHolder {
		middle: &[
			'N',
			'U',
			'R'
		],
		end: &[
			'N',
			'U',
			'R'
		]
	},
	"EN" => DiHolder {
		middle: &[
			'T',
			'C',
			'D',
			'S',
			'E',
			'I',
			'G',
			'O',
			'N',
			'A'
		],
		end: &[
			'T',
			'D',
			'S',
			'E',
			'G',
			'O',
			'A'
		]
	},
	"TI" => DiHolder {
		middle: &[
			'O',
			'N',
			'C',
			'V',
			'M',
			'L',
			'E',
			'T',
			'S',
			'A',
			'R',
			'F'
		],
		end: &[
			'N',
			'C',
			'M',
			'L',
			'E',
			'T',
			'S',
			'A',
			'R',
			'F'
		]
	},
	"FO" => DiHolder {
		middle: &[
			'R',
			'U',
			'O',
			'L'
		],
		end: &[
			'R',
			'U',
			'O',
			'L'
		]
	},
	"HE" => DiHolder {
		middle: &[
			'R',
			'N',
			'Y',
			'S',
			'M',
			'I',
			'A',
			'L',
			'D',
			'T'
		],
		end: &[
			'R',
			'N',
			'Y',
			'S',
			'M',
			'A',
			'L',
			'D',
			'T'
		]
	},
	"HA" => DiHolder {
		middle: &[
			'T',
			'D',
			'V',
			'N',
			'S',
			'R',
			'P',
			'L'
		],
		end: &[
			'T',
			'D',
			'N',
			'S',
			'R',
			'L'
		]
	},
	"HI" => DiHolder {
		middle: &[
			'S',
			'N',
			'C',
			'M',
			'L',
			'P',
			'G',
			'T',
			'R',
			'E'
		],
		end: &[
			'S',
			'N',
			'C',
			'M',
			'L',
			'P',
			'G',
			'T',
			'R',
			'E'
		]
	},
	"TE" => DiHolder {
		middle: &[
			'R',
			'D',
			'N',
			'S',
			'M',
			'L',
			'E',
			'C',
			'A'
		],
		end: &[
			'R',
			'D',
			'N',
			'S',
			'M',
			'L',
			'E',
			'A'
		]
	},
	"AT" => DiHolder {
		middle: &[
			'I',
			'E',
			'T',
			'H',
			'U',
			'O',
			'C'
		],
		end: &[
			'E',
			'H',
			'O'
		]
	},
	"ER" => DiHolder {
		middle: &[
			'E',
			'S',
			'I',
			'A',
			'N',
			'Y',
			'T',
			'V',
			'M',
			'R',
			'O',
			'L',
			'G',
			'F',
			'C'
		],
		end: &[
			'E',
			'S',
			'A',
			'N',
			'Y',
			'T',
			'M'
		]
	},
	"AL" => DiHolder {
		middle: &[
			'L',
			'S',
			'I',
			'T',
			'E',
			'U',
			'O',
			'M',
			'K',
			'F',
			'A'
		],
		end: &[
			'L',
			'S',
			'T',
			'E',
			'F'
		]
	},
	"WA" => DiHolder {
		middle: &[
			'S',
			'Y',
			'R',
			'T',
			'N',
			'L'
		],
		end: &[
			'S',
			'Y',
			'R',
			'T',
			'N',
			'L'
		]
	},
	"VE" => DiHolder {
		middle: &[
			'R',
			'N',
			'L',
			'S',
			'D'
		],
		end: &[
			'R',
			'N',
			'L',
			'S',
			'D'
		]
	},
	"CO" => DiHolder {
		middle: &[
			'N',
			'M',
			'U',
			'R',
			'L',
			'V',
			'S',
			'O'
		],
		end: &[
			'N',
			'M',
			'U',
			'R',
			'L',
			'O'
		]
	},
	"RE" => DiHolder {
		middle: &[
			'S',
			'A',
			'D',
			'N',
			'E',
			'C',
			'L',
			'T',
			'P',
			'M',
			'V',
			'G',
			'F',
			'Q'
		],
		end: &[
			'S',
			'A',
			'D',
			'N',
			'E',
			'L',
			'T',
			'P',
			'M'
		]
	},
	"IT" => DiHolder {
		middle: &[
			'H',
			'I',
			'Y',
			'E',
			'S',
			'T',
			'A',
			'U'
		],
		end: &[
			'H',
			'Y',
			'E',
			'S',
			'A'
		]
	},
	"WI" => DiHolder {
		middle: &[
			'T',
			'L',
			'N',
			'S'
		],
		end: &[
			'T',
			'L',
			'N',
			'S'
		]
	},
	"ME" => DiHolder {
		middle: &[
			'N',
			'R',
			'D',
			'T',
			'S',
			'M',
			'A'
		],
		end: &[
			'N',
			'R',
			'D',
			'T',
			'S',
			'M',
			'A'
		]
	},
	"NC" => DiHolder {
		middle: &[
			'E',
			'I',
			'H',
			'T',
			'R',
			'O',
			'L'
		],
		end: &[
			'E',
			'H',
			'T'
		]
	},
	"ON" => DiHolder {
		middle: &[
			'S',
			'E',
			'T',
			'G',
			'A',
			'D',
			'L',
			'C',
			'V',
			'O',
			'I',
			'F'
		],
		end: &[
			'S',
			'E',
			'T',
			'G',
			'A',
			'D',
			'O'
		]
	},
	"PR" => DiHolder {
		middle: &[
			'O',
			'E',
			'I',
			'A'
		],
		end: &[
			'E',
			'A'
		]
	},
	"AR" => DiHolder {
		middle: &[
			'E',
			'T',
			'D',
			'Y',
			'S',
			'I',
			'R',
			'L',
			'M',
			'K',
			'G',
			'A',
			'O',
			'N',
			'C'
		],
		end: &[
			'E',
			'T',
			'D',
			'Y',
			'S',
			'M',
			'K',
			'A',
			'N'
		]
	},
	"ES" => DiHolder {
		middle: &[
			'S',
			'T',
			'E',
			'I',
			'P',
			'U',
			'C'
		],
		end: &[
			'S',
			'T',
			'E'
		]
	},
	"EV" => DiHolder {
		middle: &[
			'E',
			'I'
		],
		end: &[
			'E'
		]
	},
	"ST" => DiHolder {
		middle: &[
			'A',
			'R',
			'I',
			'E',
			'O',
			'U',
			'S'
		],
		end: &[
			'A',
			'E',
			'O',
			'S'
		]
	},
	"EA" => DiHolder {
		middle: &[
			'R',
			'S',
			'T',
			'D',
			'L',
			'C',
			'N',
			'V',
			'M',
			'K'
		],
		end: &[
			'R',
			'S',
			'T',
			'D',
			'L',
			'N',
			'M'
		]
	},
	"IV" => DiHolder {
		middle: &[
			'E',
			'I',
			'A'
		],
		end: &[
			'E'
		]
	},
	"EC" => DiHolder {
		middle: &[
			'T',
			'O',
			'I',
			'E',
			'A',
			'U',
			'R',
			'H'
		],
		end: &[
			'T',
			'E',
			'H'
		]
	},
	"NO" => DiHolder {
		middle: &[
			'T',
			'W',
			'R',
			'U',
			'N',
			'M'
		],
		end: &[
			'T',
			'W',
			'R',
			'U',
			'N',
			'M'
		]
	},
	"OU" => DiHolder {
		middle: &[
			'T',
			'L',
			'R',
			'N',
			'S',
			'G',
			'P',
			'B'
		],
		end: &[
			'T',
			'L',
			'R',
			'N',
			'S',
			'P'
		]
	},
	"PE" => DiHolder {
		middle: &[
			'R',
			'N',
			'C',
			'A',
			'D',
			'T',
			'O'
		],
		end: &[
			'R',
			'N',
			'A',
			'D',
			'T'
		]
	},
	"IL" => DiHolder {
		middle: &[
			'L',
			'E',
			'I',
			'Y',
			'D',
			'A'
		],
		end: &[
			'L',
			'E',
			'Y',
			'D'
		]
	},
	"IS" => DiHolder {
		middle: &[
			'T',
			'H',
			'S',
			'I',
			'E',
			'C',
			'M'
		],
		end: &[
			'T',
			'H',
			'S',
			'E',
			'M'
		]
	},
	"MA" => DiHolder {
		middle: &[
			'N',
			'T',
			'L',
			'K',
			'D',
			'S',
			'I',
			'G'
		],
		end: &[
			'N',
			'T',
			'L',
			'D',
			'S'
		]
	},
	"AV" => DiHolder {
		middle: &[
			'E',
			'I',
			'A'
		],
		end: &[
			'E'
		]
	},
	"OM" => DiHolder {
		middle: &[
			'E',
			'P',
			'M',
			'I',
			'A'
		],
		end: &[
			'E'
		]
	},
	"IC" => DiHolder {
		middle: &[
			'A',
			'H',
			'E',
			'I',
			'T',
			'K',
			'U',
			'S'
		],
		end: &[
			'H',
			'E',
			'T',
			'K',
			'S'
		]
	},
	"GH" => DiHolder {
		middle: &[
			'T'
		],
		end: &[
			'T'
		]
	},
	"DE" => DiHolder {
		middle: &[
			'R',
			'N',
			'S',
			'D',
			'A',
			'V',
			'P',
			'T',
			'M',
			'L',
			'F'
		],
		end: &[
			'R',
			'N',
			'S',
			'D',
			'A',
			'P',
			'T',
			'M',
			'L'
		]
	},
	"AI" => DiHolder {
		middle: &[
			'N',
			'D',
			'R',
			'L',
			'T'
		],
		end: &[
			'N',
			'D',
			'R',
			'L',
			'T'
		]
	},
	"CT" => DiHolder {
		middle: &[
			'I',
			'E',
			'U',
			'S',
			'O'
		],
		end: &[
			'E',
			'S',
			'O'
		]
	},
	"IG" => DiHolder {
		middle: &[
			'H',
			'N',
			'I'
		],
		end: &[
			'H',
			'N'
		]
	},
	"ID" => DiHolder {
		middle: &[
			'E'
		],
		end: &[
			'E'
		]
	},
	"OR" => DiHolder {
		middle: &[
			'E',
			'T',
			'M',
			'D',
			'S',
			'K',
			'I',
			'Y',
			'L',
			'G',
			'A',
			'R',
			'N',
			'C'
		],
		end: &[
			'E',
			'T',
			'M',
			'D',
			'S',
			'K',
			'Y',
			'A',
			'N'
		]
	},
	"OV" => DiHolder {
		middle: &[
			'E',
			'I'
		],
		end: &[
			'E'
		]
	},
	"UL" => DiHolder {
		middle: &[
			'D',
			'T',
			'A',
			'L'
		],
		end: &[
			'D',
			'T',
			'L'
		]
	},
	"YO" => DiHolder {
		middle: &[
			'U'
		],
		end: &[
			'U'
		]
	},
	"BU" => DiHolder {
		middle: &[
			'T',
			'S',
			'R',
			'I'
		],
		end: &[
			'T',
			'S',
			'R'
		]
	},
	"RA" => DiHolder {
		middle: &[
			'T',
			'N',
			'L',
			'C',
			'I',
			'M',
			'D',
			'S',
			'R',
			'P',
			'G',
			'B'
		],
		end: &[
			'T',
			'N',
			'L',
			'M',
			'D',
			'S',
			'R'
		]
	},
	"FR" => DiHolder {
		middle: &[
			'O',
			'E',
			'A'
		],
		end: &[
			'E',
			'A'
		]
	},
	"RO" => DiHolder {
		middle: &[
			'M',
			'U',
			'V',
			'P',
			'N',
			'W',
			'S',
			'O',
			'L',
			'D',
			'C',
			'B',
			'A',
			'T',
			'G'
		],
		end: &[
			'M',
			'U',
			'P',
			'N',
			'W',
			'O',
			'L',
			'D',
			'T'
		]
	},
	"WH" => DiHolder {
		middle: &[
			'I',
			'E',
			'O',
			'A'
		],
		end: &[
			'E',
			'O'
		]
	},
	"OT" => DiHolder {
		middle: &[
			'H',
			'E',
			'T',
			'I'
		],
		end: &[
			'H',
			'E'
		]
	},
	"BL" => DiHolder {
		middle: &[
			'E',
			'I',
			'Y',
			'O',
			'A'
		],
		end: &[
			'E',
			'Y'
		]
	},
	"NT" => DiHolder {
		middle: &[
			'E',
			'I',
			'S',
			'R',
			'O',
			'A',
			'L',
			'Y',
			'U',
			'H'
		],
		end: &[
			'E',
			'S',
			'O',
			'A',
			'Y',
			'H'
		]
	},
	"UN" => DiHolder {
		middle: &[
			'D',
			'T',
			'I',
			'C',
			'G'
		],
		end: &[
			'D',
			'T',
			'G'
		]
	},
	"TR" => DiHolder {
		middle: &[
			'A',
			'I',
			'O',
			'E',
			'U',
			'Y'
		],
		end: &[
			'A',
			'E',
			'Y'
		]
	},
	"HO" => DiHolder {
		middle: &[
			'U',
			'W',
			'S',
			'R',
			'L',
			'O',
			'M',
			'T',
			'P',
			'N',
			'D'
		],
		end: &[
			'U',
			'W',
			'R',
			'L',
			'O',
			'M',
			'T',
			'P',
			'N',
			'D'
		]
	},
	"AC" => DiHolder {
		middle: &[
			'T',
			'E',
			'K',
			'H',
			'C',
			'R',
			'I'
		],
		end: &[
			'T',
			'E',
			'K',
			'H'
		]
	},
	"TU" => DiHolder {
		middle: &[
			'R',
			'D',
			'A',
			'T'
		],
		end: &[
			'R',
			'T'
		]
	},
	"WE" => DiHolder {
		middle: &[
			'R',
			'L',
			'E',
			'V',
			'S',
			'N',
			'A'
		],
		end: &[
			'R',
			'L',
			'E',
			'S',
			'N',
			'A'
		]
	},
	"CA" => DiHolder {
		middle: &[
			'L',
			'N',
			'T',
			'R',
			'U',
			'S',
			'M',
			'P'
		],
		end: &[
			'L',
			'N',
			'T',
			'R',
			'S',
			'M'
		]
	},
	"SH" => DiHolder {
		middle: &[
			'E',
			'O',
			'I',
			'A'
		],
		end: &[
			'E',
			'O'
		]
	},
	"UR" => DiHolder {
		middle: &[
			'E',
			'N',
			'T',
			'S',
			'I',
			'A',
			'Y',
			'R',
			'P',
			'C'
		],
		end: &[
			'E',
			'N',
			'T',
			'S',
			'A',
			'Y'
		]
	},
	"IE" => DiHolder {
		middle: &[
			'S',
			'N',
			'D',
			'T',
			'W',
			'V',
			'R',
			'L',
			'F'
		],
		end: &[
			'S',
			'N',
			'D',
			'T',
			'W',
			'R',
			'L'
		]
	},
	"PA" => DiHolder {
		middle: &[
			'R',
			'T',
			'S',
			'N',
			'L',
			'I',
			'C'
		],
		end: &[
			'R',
			'T',
			'S',
			'N',
			'L'
		]
	},
	"TO" => DiHolder {
		middle: &[
			'R',
			'O',
			'N',
			'W',
			'P',
			'M',
			'L'
		],
		end: &[
			'R',
			'O',
			'N',
			'W',
			'P',
			'M',
			'L'
		]
	},
	"EE" => DiHolder {
		middle: &[
			'N',
			'D',
			'T',
			'M',
			'S',
			'R',
			'P',
			'L',
			'K'
		],
		end: &[
			'N',
			'D',
			'T',
			'M',
			'S',
			'R',
			'P',
			'L',
			'K'
		]
	},
	"LI" => DiHolder {
		middle: &[
			'N',
			'T',
			'S',
			'C',
			'K',
			'G',
			'E',
			'F',
			'Z',
			'V',
			'O',
			'M',
			'A'
		],
		end: &[
			'N',
			'T',
			'S',
			'C',
			'G',
			'E',
			'F',
			'M',
			'A'
		]
	},
	"RI" => DiHolder {
		middle: &[
			'N',
			'E',
			'C',
			'T',
			'S',
			'G',
			'A',
			'V',
			'O',
			'P',
			'M',
			'L',
			'D',
			'B'
		],
		end: &[
			'N',
			'E',
			'C',
			'T',
			'S',
			'G',
			'A',
			'P',
			'M',
			'L',
			'D'
		]
	},
	"UG" => DiHolder {
		middle: &[
			'H',
			'G'
		],
		end: &[
			'H'
		]
	},
	"AM" => DiHolder {
		middle: &[
			'E',
			'P',
			'I',
			'O',
			'A'
		],
		end: &[
			'E'
		]
	},
	"ND" => DiHolder {
		middle: &[
			'E',
			'I',
			'S',
			'A',
			'U',
			'O'
		],
		end: &[
			'E',
			'S',
			'O'
		]
	},
	"US" => DiHolder {
		middle: &[
			'E',
			'T',
			'I',
			'S',
			'L',
			'H'
		],
		end: &[
			'E',
			'T',
			'S',
			'H'
		]
	},
	"LL" => DiHolder {
		middle: &[
			'Y',
			'E',
			'O',
			'I',
			'S',
			'A'
		],
		end: &[
			'Y',
			'E',
			'S'
		]
	},
	"AS" => DiHolder {
		middle: &[
			'T',
			'S',
			'E',
			'I',
			'U',
			'O',
			'K',
			'H'
		],
		end: &[
			'T',
			'S',
			'E',
			'O',
			'H'
		]
	},
	"TA" => DiHolder {
		middle: &[
			'T',
			'N',
			'L',
			'I',
			'R',
			'K',
			'B',
			'G',
			'C'
		],
		end: &[
			'T',
			'N',
			'L',
			'R'
		]
	},
	"LE" => DiHolder {
		middle: &[
			'S',
			'D',
			'A',
			'T',
			'C',
			'R',
			'N',
			'M',
			'G',
			'V',
			'F'
		],
		end: &[
			'S',
			'D',
			'A',
			'T',
			'R',
			'N',
			'M'
		]
	},
	"MO" => DiHolder {
		middle: &[
			'R',
			'S',
			'V',
			'T',
			'U',
			'D'
		],
		end: &[
			'R',
			'T',
			'U',
			'D'
		]
	},
	"WO" => DiHolder {
		middle: &[
			'R',
			'U'
		],
		end: &[
			'R',
			'U'
		]
	},
	"MI" => DiHolder {
		middle: &[
			'N',
			'L',
			'S',
			'T',
			'C',
			'G'
		],
		end: &[
			'N',
			'L',
			'S',
			'T',
			'C',
			'G'
		]
	},
	"AB" => DiHolder {
		middle: &[
			'L',
			'O',
			'I'
		],
		end: &[]
	},
	"EL" => DiHolder {
		middle: &[
			'L',
			'Y',
			'I',
			'E',
			'F',
			'O',
			'A',
			'T',
			'S',
			'P',
			'D'
		],
		end: &[
			'L',
			'Y',
			'E',
			'F',
			'T',
			'S',
			'D'
		]
	},
	"IA" => DiHolder {
		middle: &[
			'L',
			'N',
			'T'
		],
		end: &[
			'L',
			'N',
			'T'
		]
	},
	"NA" => DiHolder {
		middle: &[
			'L',
			'T',
			'R',
			'N',
			'M'
		],
		end: &[
			'L',
			'T',
			'R',
			'N',
			'M'
		]
	},
	"SS" => DiHolder {
		middle: &[
			'I',
			'E',
			'U',
			'O',
			'A'
		],
		end: &[
			'E',
			'O'
		]
	},
	"AG" => DiHolder {
		middle: &[
			'E',
			'A',
			'O'
		],
		end: &[
			'E',
			'O'
		]
	},
	"TT" => DiHolder {
		middle: &[
			'E',
			'L',
			'I'
		],
		end: &[
			'E'
		]
	},
	"NE" => DiHolder {
		middle: &[
			'D',
			'S',
			'W',
			'R',
			'E',
			'Y',
			'V',
			'T',
			'L',
			'C',
			'A'
		],
		end: &[
			'D',
			'S',
			'W',
			'R',
			'E',
			'Y',
			'T',
			'L',
			'A'
		]
	},
	"PL" => DiHolder {
		middle: &[
			'A',
			'E',
			'I',
			'Y',
			'O'
		],
		end: &[
			'E',
			'Y'
		]
	},
	"LA" => DiHolder {
		middle: &[
			'T',
			'N',
			'R',
			'S',
			'C',
			'Y',
			'W',
			'I',
			'B'
		],
		end: &[
			'T',
			'N',
			'R',
			'S',
			'Y',
			'W'
		]
	},
	"OS" => DiHolder {
		middle: &[
			'T',
			'E',
			'S',
			'I'
		],
		end: &[
			'T',
			'E',
			'S'
		]
	},
	"CE" => DiHolder {
		middle: &[
			'S',
			'N',
			'R',
			'D',
			'P',
			'L',
			'I'
		],
		end: &[
			'S',
			'N',
			'R',
			'D',
			'P',
			'L'
		]
	},
	"DI" => DiHolder {
		middle: &[
			'S',
			'N',
			'T',
			'D',
			'F',
			'E',
			'C',
			'A',
			'V',
			'R'
		],
		end: &[
			'S',
			'N',
			'T',
			'D',
			'F',
			'E',
			'C',
			'A',
			'R'
		]
	},
	"BE" => DiHolder {
		middle: &[
			'R',
			'E',
			'C',
			'T',
			'L',
			'F',
			'S',
			'I',
			'G',
			'D',
			'A'
		],
		end: &[
			'R',
			'E',
			'T',
			'L',
			'S',
			'D',
			'A'
		]
	},
	"AP" => DiHolder {
		middle: &[
			'P',
			'E',
			'A'
		],
		end: &[
			'E'
		]
	},
	"SI" => DiHolder {
		middle: &[
			'O',
			'N',
			'D',
			'T',
			'S',
			'G',
			'C',
			'B',
			'V',
			'M',
			'A'
		],
		end: &[
			'N',
			'D',
			'T',
			'S',
			'G',
			'C',
			'M',
			'A'
		]
	},
	"NI" => DiHolder {
		middle: &[
			'N',
			'T',
			'S',
			'C',
			'Z',
			'O',
			'G',
			'F'
		],
		end: &[
			'N',
			'T',
			'S',
			'C',
			'G',
			'F'
		]
	},
	"OW" => DiHolder {
		middle: &[
			'N',
			'E',
			'S',
			'I',
			'A'
		],
		end: &[
			'N',
			'E',
			'S'
		]
	},
	"SO" => DiHolder {
		middle: &[
			'N',
			'M',
			'U',
			'L',
			'C',
			'R'
		],
		end: &[
			'N',
			'M',
			'U',
			'L',
			'R'
		]
	},
	"AK" => DiHolder {
		middle: &[
			'E',
			'I'
		],
		end: &[
			'E'
		]
	},
	"CH" => DiHolder {
		middle: &[
			'E',
			'A',
			'I',
			'O',
			'U',
			'R'
		],
		end: &[
			'E',
			'O'
		]
	},
	"EM" => DiHolder {
		middle: &[
			'E',
			'S',
			'P',
			'O',
			'B',
			'A',
			'I'
		],
		end: &[
			'E',
			'S'
		]
	},
	"IM" => DiHolder {
		middle: &[
			'E',
			'P',
			'I',
			'A',
			'S',
			'M'
		],
		end: &[
			'E',
			'S'
		]
	},
	"SE" => DiHolder {
		middle: &[
			'D',
			'N',
			'L',
			'S',
			'R',
			'E',
			'C',
			'T',
			'V',
			'A'
		],
		end: &[
			'D',
			'N',
			'L',
			'S',
			'R',
			'E',
			'T',
			'A'
		]
	},
	"NS" => DiHolder {
		middle: &[
			'T',
			'I',
			'E'
		],
		end: &[
			'T',
			'E'
		]
	},
	"PO" => DiHolder {
		middle: &[
			'S',
			'R',
			'N',
			'L',
			'W',
			'T',
			'I'
		],
		end: &[
			'R',
			'N',
			'L',
			'W',
			'T'
		]
	},
	"EI" => DiHolder {
		middle: &[
			'R',
			'N',
			'G',
			'T'
		],
		end: &[
			'R',
			'N',
			'G',
			'T'
		]
	},
	"EX" => DiHolder {
		middle: &[
			'P',
			'T',
			'I',
			'C',
			'A'
		],
		end: &[
			'T'
		]
	},
	"KI" => DiHolder {
		middle: &[
			'N'
		],
		end: &[
			'N'
		]
	},
	"UC" => DiHolder {
		middle: &[
			'H',
			'T',
			'K',
			'E'
		],
		end: &[
			'H',
			'T',
			'K',
			'E'
		]
	},
	"AD" => DiHolder {
		middle: &[
			'E',
			'I',
			'Y',
			'V',
			'M',
			'D'
		],
		end: &[
			'E',
			'Y'
		]
	},
	"GR" => DiHolder {
		middle: &[
			'E',
			'A',
			'O'
		],
		end: &[
			'E',
			'A'
		]
	},
	"IR" => DiHolder {
		middle: &[
			'E',
			'S',
			'T',
			'L',
			'I'
		],
		end: &[
			'E',
			'S',
			'T'
		]
	},
	"NG" => DiHolder {
		middle: &[
			'E',
			'S',
			'L',
			'T',
			'R',
			'I'
		],
		end: &[
			'E',
			'S'
		]
	},
	"OP" => DiHolder {
		middle: &[
			'E',
			'P',
			'L'
		],
		end: &[
			'E'
		]
	},
	"SP" => DiHolder {
		middle: &[
			'E',
			'O',
			'I',
			'A'
		],
		end: &[
			'E'
		]
	},
	"OL" => DiHolder {
		middle: &[
			'D',
			'L',
			'I',
			'O',
			'E',
			'U'
		],
		end: &[
			'D',
			'L',
			'E'
		]
	},
	"DA" => DiHolder {
		middle: &[
			'Y',
			'T',
			'R',
			'N'
		],
		end: &[
			'Y',
			'T',
			'R',
			'N'
		]
	},
	"NL" => DiHolder {
		middle: &[
			'Y'
		],
		end: &[
			'Y'
		]
	},
	"TL" => DiHolder {
		middle: &[
			'Y',
			'E'
		],
		end: &[
			'Y',
			'E'
		]
	},
	"LO" => DiHolder {
		middle: &[
			'W',
			'N',
			'O',
			'S',
			'C',
			'V',
			'U',
			'T',
			'R',
			'P',
			'G'
		],
		end: &[
			'W',
			'N',
			'O',
			'U',
			'T',
			'R',
			'P'
		]
	},
	"BO" => DiHolder {
		middle: &[
			'U',
			'T',
			'R',
			'O',
			'D',
			'A'
		],
		end: &[
			'U',
			'T',
			'R',
			'O',
			'D'
		]
	},
	"RS" => DiHolder {
		middle: &[
			'T',
			'E',
			'O',
			'I'
		],
		end: &[
			'T',
			'E',
			'O'
		]
	},
	"FE" => DiHolder {
		middle: &[
			'R',
			'E',
			'W',
			'L',
			'C',
			'A'
		],
		end: &[
			'R',
			'E',
			'W',
			'L',
			'A'
		]
	},
	"FI" => DiHolder {
		middle: &[
			'R',
			'N',
			'C',
			'E',
			'L',
			'G'
		],
		end: &[
			'R',
			'N',
			'C',
			'E',
			'L',
			'G'
		]
	},
	"SU" => DiHolder {
		middle: &[
			'R',
			'C',
			'P',
			'B',
			'M',
			'L',
			'A'
		],
		end: &[
			'R',
			'P',
			'M',
			'L'
		]
	},
	"GE" => DiHolder {
		middle: &[
			'N',
			'T',
			'S',
			'R',
			'D'
		],
		end: &[
			'N',
			'T',
			'S',
			'R',
			'D'
		]
	},
	"MP" => DiHolder {
		middle: &[
			'L',
			'O',
			'A',
			'T',
			'R',
			'E'
		],
		end: &[
			'T',
			'E'
		]
	},
	"UA" => DiHolder {
		middle: &[
			'L',
			'T',
			'R'
		],
		end: &[
			'L',
			'T',
			'R'
		]
	},
	"OO" => DiHolder {
		middle: &[
			'K',
			'D',
			'L',
			'T',
			'R',
			'N',
			'M'
		],
		end: &[
			'K',
			'D',
			'L',
			'T',
			'R',
			'N',
			'M'
		]
	},
	"RT" => DiHolder {
		middle: &[
			'I',
			'H',
			'A',
			'E',
			'Y',
			'U',
			'S'
		],
		end: &[
			'H',
			'A',
			'E',
			'Y',
			'S'
		]
	},
	"SA" => DiHolder {
		middle: &[
			'I',
			'M',
			'Y',
			'N',
			'L'
		],
		end: &[
			'M',
			'Y',
			'N',
			'L'
		]
	},
	"CR" => DiHolder {
		middle: &[
			'E',
			'I',
			'O',
			'A'
		],
		end: &[
			'E',
			'A'
		]
	},
	"FF" => DiHolder {
		middle: &[
			'E',
			'I'
		],
		end: &[
			'E'
		]
	},
	"IK" => DiHolder {
		middle: &[
			'E'
		],
		end: &[
			'E'
		]
	},
	"MB" => DiHolder {
		middle: &[
			'E'
		],
		end: &[
			'E'
		]
	},
	"KE" => DiHolder {
		middle: &[
			'D',
			'N',
			'T',
			'S',
			'R',
			'E'
		],
		end: &[
			'D',
			'N',
			'T',
			'S',
			'R',
			'E'
		]
	},
	"FA" => DiHolder {
		middle: &[
			'C',
			'R',
			'M',
			'I'
		],
		end: &[
			'R',
			'M'
		]
	},
	"CI" => DiHolder {
		middle: &[
			'A',
			'T',
			'E',
			'S',
			'P',
			'N'
		],
		end: &[
			'A',
			'T',
			'E',
			'S',
			'P',
			'N'
		]
	},
	"EQ" => DiHolder {
		middle: &[
			'U'
		],
		end: &[]
	},
	"AF" => DiHolder {
		middle: &[
			'T',
			'F'
		],
		end: &[
			'T',
			'F'
		]
	},
	"ET" => DiHolder {
		middle: &[
			'T',
			'I',
			'H',
			'E',
			'Y',
			'W',
			'S',
			'A'
		],
		end: &[
			'H',
			'E',
			'Y',
			'S',
			'A'
		]
	},
	"AY" => DiHolder {
		middle: &[
			'S',
			'E'
		],
		end: &[
			'S'
		]
	},
	"MU" => DiHolder {
		middle: &[
			'S',
			'N',
			'L',
			'C'
		],
		end: &[
			'S',
			'N',
			'L'
		]
	},
	"UE" => DiHolder {
		middle: &[
			'S',
			'N'
		],
		end: &[
			'S',
			'N'
		]
	},
	"HR" => DiHolder {
		middle: &[
			'O',
			'E',
			'I'
		],
		end: &[
			'E'
		]
	},
	"TW" => DiHolder {
		middle: &[
			'O',
			'E'
		],
		end: &[
			'O',
			'E'
		]
	},
	"GI" => DiHolder {
		middle: &[
			'N',
			'V',
			'O',
			'C'
		],
		end: &[
			'N',
			'C'
		]
	},
	"OI" => DiHolder {
		middle: &[
			'N'
		],
		end: &[
			'N'
		]
	},
	"VI" => DiHolder {
		middle: &[
			'N',
			'D',
			'S',
			'C',
			'T',
			'O',
			'L',
			'E'
		],
		end: &[
			'N',
			'D',
			'S',
			'C',
			'T',
			'L',
			'E'
		]
	},
	"CU" => DiHolder {
		middle: &[
			'L',
			'R',
			'T',
			'S'
		],
		end: &[
			'L',
			'R',
			'T',
			'S'
		]
	},
	"FU" => DiHolder {
		middle: &[
			'L',
			'R',
			'N'
		],
		end: &[
			'L',
			'R',
			'N'
		]
	},
	"ED" => DiHolder {
		middle: &[
			'I',
			'U',
			'E'
		],
		end: &[
			'E'
		]
	},
	"QU" => DiHolder {
		middle: &[
			'I',
			'E',
			'A'
		],
		end: &[
			'E'
		]
	},
	"UT" => DiHolder {
		middle: &[
			'I',
			'H',
			'E'
		],
		end: &[
			'H',
			'E'
		]
	},
	"RC" => DiHolder {
		middle: &[
			'H',
			'E'
		],
		end: &[
			'H',
			'E'
		]
	},
	"OF" => DiHolder {
		middle: &[
			'F',
			'T'
		],
		end: &[
			'F',
			'T'
		]
	},
	"CL" => DiHolder {
		middle: &[
			'E',
			'A',
			'U',
			'O'
		],
		end: &[
			'E'
		]
	},
	"FT" => DiHolder {
		middle: &[
			'E'
		],
		end: &[
			'E'
		]
	},
	"IZ" => DiHolder {
		middle: &[
			'E',
			'A'
		],
		end: &[
			'E'
		]
	},
	"PP" => DiHolder {
		middle: &[
			'E',
			'O',
			'R',
			'L'
		],
		end: &[
			'E'
		]
	},
	"RG" => DiHolder {
		middle: &[
			'E',
			'A'
		],
		end: &[
			'E'
		]
	},
	"DU" => DiHolder {
		middle: &[
			'C',
			'S',
			'R',
			'A'
		],
		end: &[
			'S',
			'R'
		]
	},
	"RM" => DiHolder {
		middle: &[
			'A',
			'S',
			'I',
			'E'
		],
		end: &[
			'S',
			'E'
		]
	},
	"YE" => DiHolder {
		middle: &[
			'A',
			'S',
			'D'
		],
		end: &[
			'A',
			'S',
			'D'
		]
	},
	"RL" => DiHolder {
		middle: &[
			'Y',
			'D'
		],
		end: &[
			'Y',
			'D'
		]
	},
	"DO" => DiHolder {
		middle: &[
			'W',
			'N',
			'M',
			'E'
		],
		end: &[
			'W',
			'N',
			'M'
		]
	},
	"AU" => DiHolder {
		middle: &[
			'T',
			'S'
		],
		end: &[
			'T',
			'S'
		]
	},
	"EP" => DiHolder {
		middle: &[
			'T',
			'O',
			'E',
			'A'
		],
		end: &[
			'T',
			'E'
		]
	},
	"BA" => DiHolder {
		middle: &[
			'S',
			'C',
			'R',
			'N',
			'L'
		],
		end: &[
			'S',
			'R',
			'N',
			'L'
		]
	},
	"JU" => DiHolder {
		middle: &[
			'S'
		],
		end: &[
			'S'
		]
	},
	"RD" => DiHolder {
		middle: &[
			'S',
			'E',
			'I'
		],
		end: &[
			'S',
			'E'
		]
	},
	"RU" => DiHolder {
		middle: &[
			'S',
			'N',
			'C'
		],
		end: &[
			'S',
			'N'
		]
	},
	"OG" => DiHolder {
		middle: &[
			'R',
			'I'
		],
		end: &[]
	},
	"BR" => DiHolder {
		middle: &[
			'O',
			'I',
			'E',
			'A'
		],
		end: &[
			'E',
			'A'
		]
	},
	"EF" => DiHolder {
		middle: &[
			'O',
			'F',
			'U',
			'T',
			'E'
		],
		end: &[
			'F',
			'T',
			'E'
		]
	},
	"KN" => DiHolder {
		middle: &[
			'O',
			'E'
		],
		end: &[
			'O',
			'E'
		]
	},
	"LS" => DiHolder {
		middle: &[
			'O'
		],
		end: &[
			'O'
		]
	},
	"GA" => DiHolder {
		middle: &[
			'N',
			'I',
			'T',
			'R'
		],
		end: &[
			'N',
			'T',
			'R'
		]
	},
	"PI" => DiHolder {
		middle: &[
			'N',
			'T',
			'R',
			'E',
			'C'
		],
		end: &[
			'N',
			'T',
			'R',
			'E',
			'C'
		]
	},
	"YI" => DiHolder {
		middle: &[
			'N'
		],
		end: &[
			'N'
		]
	},
	"BI" => DiHolder {
		middle: &[
			'L',
			'T',
			'N'
		],
		end: &[
			'L',
			'T',
			'N'
		]
	},
	"IB" => DiHolder {
		middle: &[
			'L',
			'I',
			'E'
		],
		end: &[
			'E'
		]
	},
	"UB" => DiHolder {
		middle: &[
			'L'
		],
		end: &[]
	},
	"VA" => DiHolder {
		middle: &[
			'L',
			'T',
			'R',
			'N'
		],
		end: &[
			'L',
			'T',
			'R',
			'N'
		]
	},
	"OC" => DiHolder {
		middle: &[
			'K',
			'I',
			'E',
			'C',
			'A'
		],
		end: &[
			'K',
			'E'
		]
	},
	"IF" => DiHolder {
		middle: &[
			'I',
			'F',
			'E',
			'T'
		],
		end: &[
			'F',
			'E',
			'T'
		]
	},
	"RN" => DiHolder {
		middle: &[
			'I',
			'E',
			'M',
			'A'
		],
		end: &[
			'E',
			'A'
		]
	},
	"RR" => DiHolder {
		middle: &[
			'I',
			'E',
			'Y',
			'O'
		],
		end: &[
			'E',
			'Y'
		]
	},
	"SC" => DiHolder {
		middle: &[
			'H',
			'R',
			'O',
			'I',
			'A'
		],
		end: &[
			'H'
		]
	},
	"TC" => DiHolder {
		middle: &[
			'H'
		],
		end: &[
			'H'
		]
	},
	"CK" => DiHolder {
		middle: &[
			'E'
		],
		end: &[
			'E'
		]
	},
	"DG" => DiHolder {
		middle: &[
			'E'
		],
		end: &[
			'E'
		]
	},
	"DR" => DiHolder {
		middle: &[
			'E',
			'O',
			'I',
			'A'
		],
		end: &[
			'E',
			'A'
		]
	},
	"MM" => DiHolder {
		middle: &[
			'E',
			'U',
			'I'
		],
		end: &[
			'E'
		]
	},
	"NN" => DiHolder {
		middle: &[
			'E',
			'O',
			'I'
		],
		end: &[
			'E',
			'O'
		]
	},
	"OD" => DiHolder {
		middle: &[
			'E',
			'Y',
			'U'
		],
		end: &[
			'E',
			'Y'
		]
	},
	"RV" => DiHolder {
		middle: &[
			'E',
			'I'
		],
		end: &[
			'E'
		]
	},
	"UD" => DiHolder {
		middle: &[
			'E',
			'I'
		],
		end: &[
			'E'
		]
	},
	"XP" => DiHolder {
		middle: &[
			'E'
		],
		end: &[
			'E'
		]
	},
	"JE" => DiHolder {
		middle: &[
			'C'
		],
		end: &[]
	},
	"UM" => DiHolder {
		middle: &[
			'B',
			'E'
		],
		end: &[
			'E'
		]
	},
	"EG" => DiHolder {
		middle: &[
			'A',
			'R',
			'I',
			'E'
		],
		end: &[
			'E'
		]
	},
	"DL" => DiHolder {
		middle: &[
			'Y',
			'E'
		],
		end: &[
			'Y',
			'E'
		]
	},
	"PH" => DiHolder {
		middle: &[
			'Y',
			'O',
			'I',
			'E'
		],
		end: &[
			'Y',
			'O',
			'E'
		]
	},
	"SL" => DiHolder {
		middle: &[
			'Y',
			'A'
		],
		end: &[
			'Y'
		]
	},
	"GO" => DiHolder {
		middle: &[
			'V',
			'T',
			'O'
		],
		end: &[
			'T',
			'O'
		]
	},
	"CC" => DiHolder {
		middle: &[
			'U',
			'O',
			'E'
		],
		end: &[
			'E'
		]
	},
	"LU" => DiHolder {
		middle: &[
			'T',
			'S',
			'M',
			'E',
			'D'
		],
		end: &[
			'T',
			'S',
			'M',
			'E'
		]
	},
	"OA" => DiHolder {
		middle: &[
			'T',
			'R',
			'D'
		],
		end: &[
			'T',
			'R',
			'D'
		]
	},
	"PU" => DiHolder {
		middle: &[
			'T',
			'R',
			'L',
			'B'
		],
		end: &[
			'T',
			'R',
			'L'
		]
	},
	"UI" => DiHolder {
		middle: &[
			'T',
			'R',
			'L'
		],
		end: &[
			'T',
			'R',
			'L'
		]
	},
	"YS" => DiHolder {
		middle: &[
			'T'
		],
		end: &[
			'T'
		]
	},
	"ZA" => DiHolder {
		middle: &[
			'T'
		],
		end: &[
			'T'
		]
	},
	"HU" => DiHolder {
		middle: &[
			'S',
			'R',
			'N',
			'M'
		],
		end: &[
			'S',
			'R',
			'N',
			'M'
		]
	},
	"MR" => DiHolder {
		middle: &[
			'S'
		],
		end: &[
			'S'
		]
	},
	"OE" => DiHolder {
		middle: &[
			'S'
		],
		end: &[
			'S'
		]
	},
	"SY" => DiHolder {
		middle: &[
			'S'
		],
		end: &[
			'S'
		]
	},
	"EO" => DiHolder {
		middle: &[
			'R',
			'P'
		],
		end: &[
			'R',
			'P'
		]
	},
	"TY" => DiHolder {
		middle: &[
			'P'
		],
		end: &[]
	},
	"UP" => DiHolder {
		middle: &[
			'P',
			'O'
		],
		end: &[]
	},
	"FL" => DiHolder {
		middle: &[
			'O',
			'E'
		],
		end: &[
			'E'
		]
	},
	"LM" => DiHolder {
		middle: &[
			'O'
		],
		end: &[]
	},
	"NF" => DiHolder {
		middle: &[
			'O'
		],
		end: &[]
	},
	"RP" => DiHolder {
		middle: &[
			'O'
		],
		end: &[]
	},
	"OH" => DiHolder {
		middle: &[
			'N'
		],
		end: &[]
	},
	"NU" => DiHolder {
		middle: &[
			'M'
		],
		end: &[
			'M'
		]
	},
	"XA" => DiHolder {
		middle: &[
			'M'
		],
		end: &[
			'M'
		]
	},
	"OB" => DiHolder {
		middle: &[
			'L'
		],
		end: &[]
	},
	"VO" => DiHolder {
		middle: &[
			'L'
		],
		end: &[
			'L'
		]
	},
	"DM" => DiHolder {
		middle: &[
			'I'
		],
		end: &[]
	},
	"GN" => DiHolder {
		middle: &[
			'I'
		],
		end: &[]
	},
	"LD" => DiHolder {
		middle: &[
			'I',
			'E'
		],
		end: &[
			'E'
		]
	},
	"PT" => DiHolder {
		middle: &[
			'I'
		],
		end: &[]
	},
	"SK" => DiHolder {
		middle: &[
			'I',
			'E'
		],
		end: &[
			'E'
		]
	},
	"WR" => DiHolder {
		middle: &[
			'I'
		],
		end: &[]
	},
	"JO" => DiHolder {
		middle: &[
			'H'
		],
		end: &[]
	},
	"LT" => DiHolder {
		middle: &[
			'H',
			'E'
		],
		end: &[
			'H',
			'E'
		]
	},
	"YT" => DiHolder {
		middle: &[
			'H'
		],
		end: &[
			'H'
		]
	},
	"UF" => DiHolder {
		middle: &[
			'F'
		],
		end: &[
			'F'
		]
	},
	"BJ" => DiHolder {
		middle: &[
			'E'
		],
		end: &[]
	},
	"DD" => DiHolder {
		middle: &[
			'E'
		],
		end: &[
			'E'
		]
	},
	"EY" => DiHolder {
		middle: &[
			'E'
		],
		end: &[]
	},
	"GG" => DiHolder {
		middle: &[
			'E'
		],
		end: &[
			'E'
		]
	},
	"GL" => DiHolder {
		middle: &[
			'E',
			'A'
		],
		end: &[
			'E'
		]
	},
	"GU" => DiHolder {
		middle: &[
			'E'
		],
		end: &[
			'E'
		]
	},
	"HT" => DiHolder {
		middle: &[
			'E'
		],
		end: &[
			'E'
		]
	},
	"LV" => DiHolder {
		middle: &[
			'E'
		],
		end: &[
			'E'
		]
	},
	"MS" => DiHolder {
		middle: &[
			'E'
		],
		end: &[
			'E'
		]
	},
	"NM" => DiHolder {
		middle: &[
			'E'
		],
		end: &[
			'E'
		]
	},
	"NV" => DiHolder {
		middle: &[
			'E'
		],
		end: &[
			'E'
		]
	},
	"OK" => DiHolder {
		middle: &[
			'E'
		],
		end: &[
			'E'
		]
	},
	"PM" => DiHolder {
		middle: &[
			'E'
		],
		end: &[
			'E'
		]
	},
	"RK" => DiHolder {
		middle: &[
			'E'
		],
		end: &[
			'E'
		]
	},
	"SW" => DiHolder {
		middle: &[
			'E'
		],
		end: &[
			'E'
		]
	},
	"TM" => DiHolder {
		middle: &[
			'E'
		],
		end: &[
			'E'
		]
	},
	"XC" => DiHolder {
		middle: &[
			'E'
		],
		end: &[
			'E'
		]
	},
	"ZE" => DiHolder {
		middle: &[
			'D'
		],
		end: &[
			'D'
		]
	},
	"AW" => DiHolder {
		middle: &[
			'A'
		],
		end: &[]
	},
	"SM" => DiHolder {
		middle: &[
			'A'
		],
		end: &[]
	}
};
