pub const RESPONSE_INVALID_FLAG: u8 = 128;
pub const RESPONSE_INVALID_DATA_FORMAT_FLAG: u8 = 1;
pub const RESPONSE_INVALID_GLOBAL_CATEGORY_ID_FLAG: u8 = 2;
pub const RESPONSE_INVALID_CATEGORY_CACHE_INDEX_FLAG: u8 = 3;
pub const RESPONSE_INVALID_GLOBAL_LOCATION_ID_FLAG: u8 = 4;
pub const RESPONSE_INVALID_LOCATION_CACHE_INDEX_FLAG: u8 = 5;
pub const RESPONSE_INVALID_PERIOD_ID_FLAG: u8 = 6;
pub const RESPONSE_INVALID_TIMEZONE_ID_FLAG: u8 = 7;

pub static mut INVALID_DATA_FORMAT_RESPONSE: Vec<u8> = Vec::with_capacity(1);
pub static mut INVALID_GLOBAL_CATEGORY_ID_RESPONSE: Vec<u8> = Vec::with_capacity(1);
pub static mut INVALID_CATEGORY_CACHE_INDEX_RESPONSE: Vec<u8> = Vec::with_capacity(1);
pub static mut INVALID_GLOBAL_LOCATION_ID_RESPONSE: Vec<u8> = Vec::with_capacity(1);
pub static mut INVALID_LOCATION_CACHE_INDEX_RESPONSE: Vec<u8> = Vec::with_capacity(1);
pub static mut INVALID_PERIOD_ID_RESPONSE: Vec<u8> = Vec::with_capacity(1);
pub static mut INVALID_TIMEZONE_ID_RESPONSE: Vec<u8> = Vec::with_capacity(1);

pub fn setup_response_headers() -> void {
    INVALID_DATA_FORMAT_RESPONSE.push(RESPONSE_INVALID_FLAG + RESPONSE_INVALID_DATA_FORMAT_FLAG);
    INVALID_GLOBAL_CATEGORY_ID_RESPONSE.push(RESPONSE_INVALID_FLAG + RESPONSE_INVALID_GLOBAL_CATEGORY_ID_FLAG);
    INVALID_CATEGORY_CACHE_INDEX_RESPONSE.push(RESPONSE_INVALID_FLAG + INVALID_CATEGORY_CACHE_INDEX_RESPONSE);
    INVALID_GLOBAL_LOCATION_ID_RESPONSE.push(RESPONSE_INVALID_FLAG + RESPONSE_INVALID_GLOBAL_LOCATION_ID_FLAG);
    INVALID_LOCATION_CACHE_INDEX_RESPONSE.push(RESPONSE_INVALID_FLAG + INVALID_LOCATION_CACHE_INDEX_RESPONSE);
    INVALID_PERIOD_ID_RESPONSE.push(RESPONSE_INVALID_FLAG + RESPONSE_INVALID_PERIOD_ID_FLAG);
    INVALID_TIMEZONE_ID_RESPONSE.push(RESPONSE_INVALID_FLAG + RESPONSE_INVALID_TIMEZONE_ID_FLAG);
}

pub const URL_NEXT_MONTHS_LOCATION_POLLS: &str = "0";
pub const URL_NEXT_WEEKS_LOCATION_POLLS: &str = "1";
pub const URL_TOMORROWS_LOCATION_POLLS: &str = "2";
pub const URL_DAY_AFTER_TOMORROWS_LOCATION_POLLS: &str = "3";


pub const URL_THIS_MONTHS_LOCATION_POLL_RANKINGS_BY_GLOBAL_ID: &str = "4";
pub const URL_THIS_MONTHS_LOCATION_POLL_RANKINGS_BY_CACHE_INDEX: &str = "5";
pub const URL_THIS_WEEKS_LOCATION_POLL_RANKINGS_BY_GLOBAL_ID: &str = "6";
pub const URL_THIS_WEEKS_LOCATION_POLL_RANKINGS_BY_CACHE_INDEX: &str = "7";
pub const URL_TODAYS_LOCATION_POLL_RANKINGS_BY_GLOBAL_ID: &str = "8";
pub const URL_TODAYS_LOCATION_POLL_RANKINGS_BY_CACHE_INDEX: &str = "9";


pub const URL_LAST_MONTHS_LOCATION_POLL_RANKINGS_BY_GLOBAL_ID: &str = "a";
pub const URL_LAST_MONTHS_LOCATION_POLL_RANKINGS_BY_CACHE_INDEX: &str = "b";
pub const URL_LAST_WEEKS_LOCATION_POLL_RANKINGS_BY_GLOBAL_ID: &str = "c";
pub const URL_LAST_WEEKS_LOCATION_POLL_RANKINGS_BY_CACHE_INDEX: &str = "d";
pub const URL_YESTERDAYS_LOCATION_POLL_RANKINGS_BY_GLOBAL_ID: &str = "e";
pub const URL_YESTERDAYS_LOCATION_POLL_RANKINGS_BY_CACHE_INDEX: &str = "f";
pub const URL_DAY_B4_YESTERDAY_LOCATION_POLL_RANKINGS_BY_GLOBAL_ID: &str = "g";
pub const URL_DAY_B4_YESTERDAY_LOCATION_POLL_RANKINGS_BY_CACHE_INDEX: &str = "h";




pub const URL_NEXT_MONTHS_CATEGORY_POLLS: &str = "i";
pub const URL_NEXT_WEEKS_CATEGORY_POLLS: &str = "j";
pub const URL_TOMORROWS_CATEGORY_POLLS: &str = "k";
pub const URL_DAY_AFTER_TOMORROWS_CATEGORY_POLLS: &str = "l";


pub const URL_THIS_MONTHS_CATEGORY_POLL_RANKINGS_BY_GLOBAL_ID: &str = "m";
pub const URL_THIS_MONTHS_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEX: &str = "n";
pub const URL_THIS_WEEKS_CATEGORY_POLL_RANKINGS_BY_GLOBAL_ID: &str = "o";
pub const URL_THIS_WEEKS_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEX: &str = "p";
pub const URL_TODAYS_CATEGORY_POLL_RANKINGS_BY_GLOBAL_ID: &str = "q";
pub const URL_TODAYS_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEX: &str = "r";


pub const URL_LAST_MONTHS_CATEGORY_POLL_RANKINGS_BY_GLOBAL_ID: &str = "s";
pub const URL_LAST_MONTHS_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEX: &str = "t";
pub const URL_LAST_WEEKS_CATEGORY_POLL_RANKINGS_BY_GLOBAL_ID: &str = "u";
pub const URL_LAST_WEEKS_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEX: &str = "v";
pub const URL_YESTERDAYS_CATEGORY_POLL_RANKINGS_BY_GLOBAL_ID: &str = "w";
pub const URL_YESTERDAYS_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEX: &str = "x";
pub const URL_DAY_B4_YESTERDAY_CATEGORY_POLL_RANKINGS_BY_GLOBAL_ID: &str = "y";
pub const URL_DAY_B4_YESTERDAY_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEX: &str = "z";




pub const URL_NEXT_MONTHS_LOCATION_CATEGORY_POLLS: &str = "A";
pub const URL_NEXT_WEEKS_LOCATION_CATEGORY_POLLS: &str = "B";
pub const URL_TOMORROWS_LOCATION_CATEGORY_POLLS: &str = "C";
pub const URL_DAY_AFTER_TOMORROWS_LOCATION_CATEGORY_POLLS: &str = "D";


pub const URL_THIS_MONTHS_LOCATION_CATEGORY_POLL_RANKINGS_BY_GLOBAL_IDS: &str = "E";
pub const URL_THIS_MONTHS_LOCATION_CATEGORY_POLL_RANKINGS_BY_LOCATION_CACHE_INDEX_AND_GLOBAL_CATEGORY_ID: &str = "F";
pub const URL_THIS_MONTHS_LOCATION_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEXES: &str = "G";
pub const URL_THIS_WEEKS_LOCATION_CATEGORY_POLL_RANKINGS_BY_GLOBAL_IDS: &str = "H";
pub const URL_THIS_WEEKS_LOCATION_CATEGORY_POLL_RANKINGS_BY_LOCATION_CACHE_INDEX_AND_GLOBAL_CATEGORY_ID: &str = "I";
pub const URL_THIS_WEEKS_LOCATION_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEXES: &str = "J";
pub const URL_TODAYS_LOCATION_CATEGORY_POLL_RANKINGS_BY_GLOBAL_IDS: &str = "K";
pub const URL_TODAYS_LOCATION_CATEGORY_POLL_RANKINGS_BY_LOCATION_CACHE_INDEX_AND_GLOBAL_CATEGORY_ID: &str = "L";
pub const URL_TODAYS_LOCATION_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEXES: &str = "M";


pub const URL_LAST_MONTHS_LOCATION_CATEGORY_POLL_RANKINGS_BY_GLOBAL_IDS: &str = "N";
pub const URL_LAST_MONTHS_LOCATION_CATEGORY_POLL_RANKINGS_BY_LOCATION_CACHE_INDEX_AND_GLOBAL_CATEGORY_ID: &str = "O";
pub const URL_LAST_MONTHS_LOCATION_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEXES: &str = "P";
pub const URL_LAST_WEEKS_LOCATION_CATEGORY_POLL_RANKINGS_BY_GLOBAL_IDS: &str = "Q";
pub const URL_LAST_WEEKS_LOCATION_CATEGORY_POLL_RANKINGS_BY_LOCATION_CACHE_INDEX_AND_GLOBAL_CATEGORY_ID: &str = "R";
pub const URL_LAST_WEEKS_LOCATION_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEXES: &str = "S";
pub const URL_YESTERDAYS_LOCATION_CATEGORY_POLL_RANKINGS_BY_GLOBAL_IDS: &str = "T";
pub const URL_YESTERDAYS_LOCATION_CATEGORY_POLL_RANKINGS_BY_LOCATION_CACHE_INDEX_AND_GLOBAL_CATEGORY_ID: &str = "U";
pub const URL_YESTERDAYS_LOCATION_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEXES: &str = "V";
pub const URL_DAY_B4_YESTERDAY_LOCATION_YESTERDAY_CATEGORY_POLL_RANKINGS_BY_GLOBAL_IDS: &str = "W";
pub const URL_DAY_B4_YESTERDAY_LOCATION_YESTERDAY_CATEGORY_POLL_RANKINGS_BY_LOCATION_CACHE_INDEX_AND_GLOBAL_CATEGORY_ID: &str = "X";
pub const URL_DAY_B4_YESTERDAY_LOCATION_YESTERDAY_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEXES: &str = "Y";


/**  Time zones **/
//UTC Offset	Locations	Example Name	Example Location
pub const UTC_PLUS_14: usize = 0; //	Christmas Island/Kiribati	LINT	Kiritimati
pub const UTC_PLUS_13: usize = 1;	// Tonga and 3 more	TOT	Nukualofa
pub const UTC_PLUS_12_45: usize = 2;	// Chatham Islands/New Zealand	CHAST	Chatham Islands
pub const UTC_PLUS_12: usize = 3;	// New Zealand with exceptions and 9 more	ANAT	Anadyr
pub const UTC_PLUS_11: usize = 4;	// small region of Russia and 6 more	SBT	Honiara
pub const UTC_PLUS_10_30: usize = 5;	// Lord Howe Island/Australia	LHST	Lord Howe Island
pub const UTC_PLUS_10: usize = 6;	// much of Australia and 6 more	AEST	Melbourne
pub const UTC_PLUS_9_30: usize = 7; // some regions of Australia	ACST	Adelaide
pub const UTC_PLUS_9: usize = 8; // Japan, South Korea and 5 more	JST	Tokyo
pub const UTC_PLUS_8_45: usize = 9; // Western Australia/Australia	ACWST	Eucla
pub const UTC_PLUS_8: usize = 10; // China, Philippines and 11 more	CST	Beijing
pub const UTC_PLUS_7: usize = 11; // much of Indonesia, Thailand and 7 more	WIB	Jakarta
pub const UTC_PLUS_6_30: usize = 12; // Myanmar and Cocos Islands	MMT	Yangon
pub const UTC_PLUS_6: usize = 13; // Bangladesh and 6 more	BST	Dhaka
pub const UTC_PLUS_5_45: usize = 14; // Nepal	NPT	Kathmandu
pub const UTC_PLUS_5_30: usize = 15; // India and Sri Lanka	IST	New Delhi
pub const UTC_PLUS_5: usize = 16;	// Pakistan and 8 more	UZT	Tashkent
pub const UTC_PLUS_4_30: usize = 17; // Afghanistan	AFT	Kabul
pub const UTC_PLUS_4: usize = 18; // Azerbaijan and 8 more	GST	Dubai
pub const UTC_PLUS_3_30: usize = 19; // Iran	IRST	Tehran
pub const UTC_PLUS_3: usize = 20; // Greece and 37 more	MSK	Moscow
pub const UTC_PLUS_2: usize = 21; // Germany and 47 more	CEST	Brussels
pub const UTC_PLUS_1: usize = 22; // United Kingdom and 23 more	BST	London
pub const UTC_PLUS_0: usize = 23; // Iceland and 16 more	GMT	Accra
pub const UTC_MINUS_1: usize = 24; // Cabo Verde	CVT	Praia
pub const UTC_MINUS_2: usize = 25; // most of Greenland and 3 more	WGST	Nuuk
pub const UTC_MINUS_2_30: usize = 26; // Newfoundland and Labrador/Canada	NDT	St. John's
pub const UTC_MINUS_3: usize = 27; // most of Brazil, Argentina and 9 more	ART	Buenos Aires
pub const UTC_MINUS_4: usize = 28; // regions of USA and 32 more	EDT	New York
pub const UTC_MINUS_5: usize = 29; // regions of USA and 10 more	CDT	Mexico City
pub const UTC_MINUS_6: usize = 30; // small region of USA and 9 more	CST	Guatemala City
pub const UTC_MINUS_7: usize = 31; // regions of USA and 2 more	PDT	Los Angeles
pub const UTC_MINUS_8: usize = 32; // Alaska/USA and 2 more	AKDT	Anchorage
pub const UTC_MINUS_9: usize = 33; // Alaska/USA and regions of French Polynesia	HDT	Adak
pub const UTC_MINUS_9_30: usize = 34; // Marquesas Islands/French Polynesia	MART	Taiohae
pub const UTC_MINUS_10: usize = 35; // Hawaii/USA and 2 more	HST	Honolulu
pub const UTC_MINUS_11: usize = 36; // American Samoa and 2 more	NUT	Alofi
pub const UTC_MINUS_12: usize = 37; // much of US Minor Outlying Islands	AoE	Baker Island
pub const ALL_TIME_ZONES: usize = 38; // all time zones


pub const POLL_TYPE_1D: u8 = 1;
pub const POLL_TYPE_2D: u8 = 2;
pub const POLL_TYPE_3D: u8 = 3;