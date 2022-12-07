

#[derive(Debug, Clone)]
pub enum BitsData {
    Literal(usize),
    Operator(Vec<BitsNode>),
}

#[derive(Debug, Clone)]
pub struct BitsNode {
    version: u8,
    typeid: u8,
    data: BitsData,
}

impl BitsNode {
    pub fn from_hex( hex: &str ) -> Self {
        let idx = [
            [false, false, false, false], 
            [false, false, false,  true], 
            [false, false,  true, false], 
            [false, false,  true,  true], 
            [false,  true, false, false], 
            [false,  true, false,  true], 
            [false,  true,  true, false], 
            [false,  true,  true,  true], 
            [ true, false, false, false], 
            [ true, false, false,  true], 
            [ true, false,  true, false], 
            [ true, false,  true,  true], 
            [ true,  true, false, false], 
            [ true,  true, false,  true], 
            [ true,  true,  true, false], 
            [ true,  true,  true,  true]];
        
        let input: Vec<bool> = hex.trim().bytes().map(|b| if b >= 48 && b <= 57 { b - 48 } else if b >= 65 && b <= 70 { b + 10 - 65 } else { panic!(); } ).map(|b| idx[b as usize].iter().cloned()).flatten().collect();
        
        decode_bits_node( &input, 0 ).0
    }

    pub fn sum_versions( &self ) -> usize {
        match &self.data {
            BitsData::Literal(_) => self.version as usize,
            BitsData::Operator(children) => {
                let child_sum: usize = children.iter().map(|child| child.sum_versions()).sum();
                self.version as usize + child_sum
            }
        }
    }

    pub fn eval_basic( &self ) -> usize {
        match &self.data {
            BitsData::Literal(val) => *val,
            BitsData::Operator(children) => {
                let mut child_vals = children.iter().map(|child| child.eval_basic());

                match self.typeid {
                    0 => child_vals.sum(), // sum
                    1 => child_vals.fold(1, |prod, item| prod * item), // product
                    2 => child_vals.min().unwrap(), // min
                    3 => child_vals.max().unwrap(), // max
                    5 => if child_vals.next().unwrap() > child_vals.next().unwrap() { 1 } else { 0 },
                    6 => if child_vals.next().unwrap() < child_vals.next().unwrap() { 1 } else { 0 },
                    7 => if child_vals.next().unwrap() == child_vals.next().unwrap() { 1 } else { 0 },
                    _ => panic!(),
                }
            }
        }
    }
}

fn decode_bits_node( input: &[bool], pos: usize ) -> (BitsNode, usize) {
    let version = bitfield_read(input, pos, 3);
    let typeid = bitfield_read(input, pos+3, 3);

    if typeid == 4 {
        // Literal value
        let mut pos = pos + 6;
        let mut value = 0;

        loop {
            let cont = bitfield_read(input, pos, 1);
            let chunk = bitfield_read(input, pos + 1, 4);

            value *= 16;
            value += chunk;

            pos += 5;

            if cont == 0 {
                return (BitsNode { version: version as u8, typeid: typeid as u8, data: BitsData::Literal(value) }, pos);
            }
        }
    }
    else {
        let lengthtype = bitfield_read(input, pos+6, 1);
        let mut pos = pos + 7;
        let mut subnodes = Vec::new();

        if lengthtype == 0 {
            let subpktbits = bitfield_read(input, pos, 15);

            pos += 15;
            let endpos = pos + subpktbits;

            while pos < endpos {
                let (nextnode, nextpos) = decode_bits_node(input, pos);

                subnodes.push(nextnode);
                pos = nextpos;
            }
        }
        else {
            let subpkts = bitfield_read(input, pos, 11);

            pos += 11;
            
            for _subpkt in 0..subpkts {
                let (nextnode, nextpos) = decode_bits_node(input, pos);

                subnodes.push(nextnode);
                pos = nextpos;
            }
        }

        (BitsNode { version: version as u8, typeid: typeid as u8, data: BitsData::Operator(subnodes) }, pos)
    }
}

fn bitfield_read( input: &[bool], start: usize, bits: usize ) -> usize {

    let mut field = 0;

    for bit in 0..bits {
        field *= 2;

        field += if input[start+bit] { 1 } else { 0 };
    }

    field
}

#[test]
fn test_sum_versions() {
    let input = "620D7800996600E43184312CC01A88913E1E180310FA324649CD5B9DA6BFD107003A4FDE9C718593003A5978C00A7003C400A70025400D60259D400B3002880792201B89400E601694804F1201119400C600C144008100340013440021279A5801AE93CA84C10CF3D100875401374F67F6119CA46769D8664E76FC9E4C01597748704011E4D54D7C0179B0A96431003A48ECC015C0068670FA7EF1BC5166CE440239EFC226F228129E8C1D6633596716E7D4840129C4C8CA8017FCFB943699B794210CAC23A612012EB40151006E2D4678A4200EC548CF12E4FDE9BD4A5227C600F80021D08219C1A00043A27C558AA200F4788C91A1002C893AB24F722C129BDF5121FA8011335868F1802AE82537709999796A7176254A72F8E9B9005BD600A4FD372109FA6E42D1725EDDFB64FFBD5B8D1802323DC7E0D1600B4BCDF6649252B0974AE48D4C0159392DE0034B356D626A130E44015BD80213183A93F609A7628537EB87980292A0D800F94B66546896CCA8D440109F80233ABB3ABF3CB84026B5802C00084C168291080010C87B16227CB6E454401946802735CA144BA74CFF71ADDC080282C00546722A1391549318201233003361006A1E419866200DC758330525A0C86009CC6E7F2BA00A4E7EF7AD6E873F7BD6B741300578021B94309ABE374CF7AE7327220154C3C4BD395C7E3EB756A72AC10665C08C010D0046458E72C9B372EAB280372DFE1BCA3ECC1690046513E5D5E79C235498B9002BD132451A5C78401B99AFDFE7C9A770D8A0094EDAC65031C0178AB3D8EEF8E729F2C200D26579BEDF277400A9C8FE43D3030E010C6C9A078853A431C0C0169A5CB00400010F8C9052098002191022143D30047C011100763DC71824200D4368391CA651CC0219C51974892338D0";

    let packet = BitsNode::from_hex(input);

    assert_eq!( packet.sum_versions(), 897 );
}

#[test]
fn test_eval_basic() {
    let input = "620D7800996600E43184312CC01A88913E1E180310FA324649CD5B9DA6BFD107003A4FDE9C718593003A5978C00A7003C400A70025400D60259D400B3002880792201B89400E601694804F1201119400C600C144008100340013440021279A5801AE93CA84C10CF3D100875401374F67F6119CA46769D8664E76FC9E4C01597748704011E4D54D7C0179B0A96431003A48ECC015C0068670FA7EF1BC5166CE440239EFC226F228129E8C1D6633596716E7D4840129C4C8CA8017FCFB943699B794210CAC23A612012EB40151006E2D4678A4200EC548CF12E4FDE9BD4A5227C600F80021D08219C1A00043A27C558AA200F4788C91A1002C893AB24F722C129BDF5121FA8011335868F1802AE82537709999796A7176254A72F8E9B9005BD600A4FD372109FA6E42D1725EDDFB64FFBD5B8D1802323DC7E0D1600B4BCDF6649252B0974AE48D4C0159392DE0034B356D626A130E44015BD80213183A93F609A7628537EB87980292A0D800F94B66546896CCA8D440109F80233ABB3ABF3CB84026B5802C00084C168291080010C87B16227CB6E454401946802735CA144BA74CFF71ADDC080282C00546722A1391549318201233003361006A1E419866200DC758330525A0C86009CC6E7F2BA00A4E7EF7AD6E873F7BD6B741300578021B94309ABE374CF7AE7327220154C3C4BD395C7E3EB756A72AC10665C08C010D0046458E72C9B372EAB280372DFE1BCA3ECC1690046513E5D5E79C235498B9002BD132451A5C78401B99AFDFE7C9A770D8A0094EDAC65031C0178AB3D8EEF8E729F2C200D26579BEDF277400A9C8FE43D3030E010C6C9A078853A431C0C0169A5CB00400010F8C9052098002191022143D30047C011100763DC71824200D4368391CA651CC0219C51974892338D0";

    let packet = BitsNode::from_hex(input);

    assert_eq!( packet.eval_basic(), 9485076995911 );
}
