mod Graph;

use std::io::Write;

const INTRO: &'static str = "\
\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}dP oo oo dP                  dP                     
\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}88       88                  88                     
\u{00A0}\u{00A0}.d888b88 dP dP 88  .dP  .d8888b. d8888P 88d888b. .d8888b. 
\u{00A0}\u{00A0}88'  `88 88 88 88888\"   Y8ooooo.   88   88'  `88 88'  `88 
\u{00A0}\u{00A0}88.  .88 88 88 88  `8b.       88   88   88       88.  .88 
\u{00A0}\u{00A0}`88888P8 dP 88 dP   `YP `88888P'   dP   dP       `88888P8 
\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}      88                                            
\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}      dP                                            
\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}                               dP            dP   
\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}                               88            88   
\u{00A0}\u{00A0}.d8888b. 88d888b. dP    dP 88d888b. 88 .d8888b. d8888P 
\u{00A0}\u{00A0}88'  `88 88'  `88 88    88 88'  `88 88 88'  `88   88   
\u{00A0}\u{00A0}88.  .88 88    88 88.  .88 88.  .88 88 88.  .88   88   
\u{00A0}\u{00A0}`8888P88 dP    dP `88888P' 88Y888P' dP `88888P'   dP   
\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}.88                   88                          
\u{00A0}\u{00A0}\u{00A0}d8888P                    dP                          \n
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n\n";

fn main() {
    let mut my_graph : crate::Graph::Graph;

    let matrix = my_graph.get_adjacency_matrix();

    for matrix in matrix {
        println!("{}", matrix);
    }


/* (Everything works in this comment)
    println!("{}", INTRO);
    print!("   GraphML-filepath: ");

    // force stream to flush
    std::io::stdout().flush().unwrap();
 
    let mut filepath = String::new();

    std::io::stdin()
        .read_line(&mut filepath)
        .expect("Error while reading...");
*/
    



    //println!("{}", filepath);
}
