// use std::io;
use serde_json::json;

fn main() {
    // Sample JSON object
    let json_object = json!({
        "person": {
            "name": "John",
            "age": 25,
            "address": {
                "city": "New York",
                "zip": "10001"
            }
        }
    });

    // Second-level keys to check
    let first_level_key = "person";
    //let second_level_key = "address";

    if json_object.get(first_level_key).is_some() {
        println!("ddddddd '{}' ddddddddddd", first_level_key);
    }

    // Using nested pattern matching
    // match json_object.get(first_level_key) {
    //     Some(inner_object) => match inner_object.get(second_level_key) {
    //         Some(second_level_value) => {
    //             // Second-level key exists, value is present
    //             println!(
    //                 "Second-level key '{}' exists with value: {:?}",
    //                 second_level_key, second_level_value
    //             );
    //         }
    //         None => {
    //             // Second-level key does not exist
    //             println!("Second-level key '{}' does not exist", second_level_key);
    //         }
    //     },
    //     None => {
    //         // First-level key does not exist
    //         println!("First-level key '{}' does not exist", first_level_key);
    //     }
    // }

    // Using chaining with the get method
    // if let Some(inner_object) = json_object.get(first_level_key) {
    //     if let Some(second_level_value) = inner_object.get(second_level_key) {
    //         // Second-level key exists, value is present
    //         println!(
    //             "Second-level key '{}' exists with value: {:?}",
    //             second_level_key, second_level_value
    //         );
    //     } else {
    //         // Second-level key does not exist
    //         println!("Second-level key '{}' does not exist", second_level_key);
    //     }
    // } else {
    //     // First-level key does not exist
    //     println!("First-level key '{}' does not exist", first_level_key);
    // }
}

// fn main()  {

    // let json_object = json!({
    //     "name": "John",
    //     "age": 25,
    //     "city": {
    //         "ram":"fff"
    //     },
    // });

    // let first_level_key = "name";
    // let second_level_key = "ram";

    // if let Some(inner_object) = json_object.get(first_level_key) {
    //     if let Some(second_level_value) = inner_object.get(second_level_key) {
    //         // Second-level key exists, value is present
    //         println!(
    //             "Second-level key '{}' exists with value: {:?}",
    //             second_level_key, second_level_value
    //         );
    //     } else {
    //         // Second-level key does not exist
    //         println!("Second-level key '{}' does not exist", second_level_key);
    //     }
    // } else {
    //     // First-level key does not exist
    //     println!("First-level key '{}' does not exist", first_level_key);
    // }

    // // println!("Enter your name:");

    //    // Original string
    //     // Original string
    //     // let original_string = "abcdefgh";

    //     // // Get the length of the original string
    //     // let len = original_string.len();
    
    //     // Check if the string has at least 5 characters
    //     // if len >= 5 {
    //     //     // Use string slicing to get the last 5 characters
    //     //     let last_five_chars = &original_string[len - 5..];
    
    //     //     // Use string slicing to get the remaining characters (excluding the last 5)
    //     //     let remaining_chars = &original_string[..len - 5];

    //     //     let result = remaining_chars.to_owned() + "." + last_five_chars;
    //     //     println!("result: {}", result);

    //     //     let xx:f64 = 1.87;
    //     //     let xxx:f64 = 1.99;

    //     //     println!("result: {}", xxx - xx);
    //     //     // Print the results
    //     //     // println!("Last five characters: {}", last_five_chars);
    //     //     // println!("Remaining characters: {}", remaining_chars);
        // } else {
        //     // Handle the case when the string has fewer than 5 characters
        //     println!("The string has fewer than 5 characters");
        // }
    

    // let mut input = String::new();

    // io::stdin()
    //     .read_line(&mut input)
    //     .expect("Failed to read line");

    //     println!("Hello, {}", input.trim());

    //     // hex::encode(&msg.data)

    //     println!("byby, {}", hex::encode(&input));

    //     let xx = "277b2264617461223a7b22646174615f696e666f223a7b22646174615f64657461696c73223a207b22616363656c65726f6d65746572223a7b2278223a2031322c202279223a2033322c20227a223a2036377d2c20226779726f73636f7065223a7b2278223a2031322c202279223a2033322c20227a223a2036377d2c226d61676e65746f6d65746572223a7b2278223a2031322c202279223a2033322c20227a223a2036377d2c20226c6f636174696f6e223a7b226c6174223a2031322c20226c6e67223a2033327d2c202274726970223a226765726d616e79222c2022636f6e7472616374223a20226b6a68222c2276656869636c655f696e666f223a7b226c6f61645f706374223a2031322c202274656d70223a2033322c202272706d223a2036372c2022767373223a2034342c2022696174223a2034342c20226d6166223a2035352c20227468726f74746c65706f223a20342c202272756e746d223a20382c2022666c69223a2038382c20226261726f223a20382c20226c6f61645f616273223a372c20226675656c5f72617465223a2039392c20226f646f6d65746572223a2039387d7d7d2c227369676e223a20223330343430323230373634626464313130656131303863663263616238333462363339373834323265666363646437616161333535636331633262643039346466333265633538343032323037386361643234636335366334323637613465343864653062333063613931663734383730376330653331616465613539363538323937353537376131643335222c227075626b6579223a20223330353933303133303630373261383634386365336430323031303630383261383634386365336430333031303730333432303030346438303335623534653063356465376137326665333966613039376262626564363832643962343039383630363439326236353934623834633236616537366337346566656565323638363636366566366535366262633736393835353765663536623365373835316237346662396137316136386262646464376362343265227d7d270a";

    //     //let gg = hex::encode(&input);

    //     let byte_vector = hex::decode(xx).expect("Failed to decode hex string");

    //     let original_string = String::from_utf8_lossy(&byte_vector);
    //     //let original_string = String::from_utf8(byte_vector).expect("Failed to convert to original string");
    //     println!("original_string-->, {}", original_string.to_string());
        //let converted_hex_string = hex::encode(&byte_vector);
        //println!("naynay, {}", converted_hex_string);
    //let json_str = r#"{ "my_field": null, "other_field": "some_value" }"#;


    // let cc = r#"{"anonymize_data": {"data":{"data_info":{"data_details": {"accelerometer":{"x": 12, "y": 32, "z": 67}, "gyroscope":{"x": 12, "y": 32, "z": 67},
    // "magnetometer":{"x": 12, "y": 32, "z": 67}, "location":{"lat": 12, "lng": 32}, "trip":"germany", "contract": "kjh",
    // "vehicle_info":{"load_pct": 12, "temp": 23, "rpm": 67, "vss": 44, "iat": 44, "maf": 55, "throttlepo": 4, "runtm": 8, "fli": 88, "baro": 8, "load_abs": 7, "fuel_rate": 99, "odometer": 98}
    // }},"sign": "30440220764bdd110ea108cf2cab834b63978422efccdd7aaa355cc1c2bd094df32ec584022078cad24cc56c4267a4e48de0b30ca91f748707c0e31adea596582975577a1d35","pubkey": "3059301306072a8648ce3d020106082a8648ce3d03010703420004d8035b54e0c5de7a72fe39fa097bbbed682d9b4098606492b6594b84c26ae76c74efeee2686666ef6e56bbc7698557ef56b3e7851b74fb9a71a68bbddd7cb42e"}}}"#;

    //let json_string = format!(r#""{}""#, json_str);


    // let hex_string = hex::encode(cc);

    // println!("encrypted--3333333333333--->: {:?}", hex_string);

    // Decode the hex string into a byte vector
    // let byte_vector = hex::decode(&hex_string).expect("Failed to decode hex string");

    // // Convert the byte vector back to a hex string
    // let converted_hex_string = hex::encode(&byte_vector);

    // if converted_hex_string != hex_string {
    //     println!("sssssssssssssssss");
    // }

    // println!("aaaaaaaaaaaaaaaaaaaaaaaa");
// }