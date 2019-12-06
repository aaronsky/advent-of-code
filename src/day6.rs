use std::collections::{HashMap, HashSet};
use std::hash::Hash;

trait InsertingIntoExistingValue<K, V> {
    fn insert_into_existing_value(&mut self, key: K, value: V);
}

type AdjacencyList<S> = HashMap<S, Vec<S>>;

impl<S> InsertingIntoExistingValue<S, S> for AdjacencyList<S>
where
    S: Hash + Eq,
{
    fn insert_into_existing_value(&mut self, key: S, value: S) {
        if let Some(mut existing) = self.remove(&key) {
            existing.push(value);
            self.insert(key, existing);
        } else {
            self.insert(key, vec![value]);
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
enum OrbitalObject {
    You,
    Santa,
    Object(String),
}

impl From<&str> for OrbitalObject {
    fn from(string: &str) -> Self {
        match string {
            "YOU" => OrbitalObject::You,
            "SAN" => OrbitalObject::Santa,
            obj => OrbitalObject::Object(obj.to_string()),
        }
    }
}

#[derive(Debug)]
struct OrbitMap {
    all_objects: HashSet<OrbitalObject>,
    lookup: AdjacencyList<OrbitalObject>,
    reverse_lookup: HashMap<OrbitalObject, OrbitalObject>,
}

impl OrbitMap {
    fn parse(input: &str) -> Self {
        let mut all_objects = HashSet::new();
        let mut lookup = HashMap::new();
        let mut reverse_lookup = HashMap::new();
        for line in input.split("\n") {
            let adjacency: Vec<&str> = line.split(")").take(2).map(str::trim).collect();
            if adjacency.len() != 2 {
                continue;
            }
            let (key, value) = (
                OrbitalObject::from(adjacency[0]),
                OrbitalObject::from(adjacency[1]),
            );
            let (reverse_key, reverse_value) = (value.clone(), key.clone());
            all_objects.insert(key.clone());
            all_objects.insert(reverse_key.clone());
            lookup.insert_into_existing_value(key, value);
            reverse_lookup.insert(reverse_key, reverse_value);
        }
        OrbitMap {
            all_objects,
            lookup,
            reverse_lookup,
        }
    }

    fn object_has_direct_orbit_to_other_object(
        &self,
        obj1: &OrbitalObject,
        obj2: &OrbitalObject,
    ) -> bool {
        if let Some(adjacency) = self.reverse_lookup.get(obj1) {
            return adjacency == obj2;
        }
        false
    }

    fn object_has_indirect_orbit_to_other_object(
        &self,
        obj1: &OrbitalObject,
        obj2: &OrbitalObject,
    ) -> bool {
        false
    }

    fn construct_orbital_path_map(&self) -> AdjacencyList<OrbitalObject> {
        let mut path_map: AdjacencyList<OrbitalObject> = Default::default();
        for obj in &self.all_objects {
            let mut current = obj;
            path_map.insert(obj.clone(), Default::default());
            while self.reverse_lookup.contains_key(current) {
                current = self.reverse_lookup.get(current).unwrap();
                path_map.insert_into_existing_value(obj.clone(), current.clone());
            }
        }
        path_map
    }

    fn number_of_orbits(&self) -> usize {
        self.construct_orbital_path_map()
            .iter()
            .map(|(_, v)| v.len())
            .sum()
    }

    fn number_of_orbital_transfers_from_you_to_santa(&self) -> usize {
        let path_map = self.construct_orbital_path_map();
        assert!(path_map.contains_key(&OrbitalObject::You));
        assert!(path_map.contains_key(&OrbitalObject::Santa));
        let you_orbits = path_map.get(&OrbitalObject::You).unwrap();
        let santa_orbits = path_map.get(&OrbitalObject::Santa).unwrap();
        let mut last_matching_you_index = you_orbits.len() - 1;
        let mut last_matching_santa_index = santa_orbits.len() - 1;
        for (reverse_index, (you, santa)) in you_orbits
            .iter()
            .rev()
            .zip(santa_orbits.iter().rev())
            .enumerate()
        {
            if you != santa {
                break;
            }
            last_matching_you_index = you_orbits.len() - reverse_index - 1;
            last_matching_santa_index = santa_orbits.len() - reverse_index - 1;
        }
        let path: Vec<&OrbitalObject> = you_orbits[..last_matching_you_index]
            .into_iter()
            .chain(santa_orbits[..=last_matching_santa_index].into_iter().rev())
            .collect();
        path.len() - 1
    }
}

/**
OrbitMap {
    lookup: {
        "E": ["F", "J"],
        "J": ["K"],
        "B": ["C", "G"],
        "COM": ["B"],
        "G": ["H"],
        "K": ["L"],
        "C": ["D"],
        "D": ["E", "I"]
    },
    reverse_lookup: {
        "L": ["K"],
        "J": ["E"],
        "I": ["D"],
        "D": ["C"],
        "G": ["B"],
        "H": ["G"],
        "F": ["E"],
        "B": ["COM"],
        "C": ["B"],
        "E": ["D"],
        "K": ["J"]
    }
}
*/

/*
{
    OrbitalObject("C"): [OrbitalObject("B"), OrbitalObject("COM")],
    OrbitalObject("E"): [OrbitalObject("D"), OrbitalObject("C"), OrbitalObject("B"), OrbitalObject("COM")],
    OrbitalObject("B"): [OrbitalObject("COM")],
    OrbitalObject("L"): [OrbitalObject("K"), OrbitalObject("J"), OrbitalObject("E"), OrbitalObject("D"), OrbitalObject("C"), OrbitalObject("B"), OrbitalObject("COM")],
    OrbitalObject("J"): [OrbitalObject("E"), OrbitalObject("D"), OrbitalObject("C"), OrbitalObject("B"), OrbitalObject("COM")],
    OrbitalObject("K"): [OrbitalObject("J"), OrbitalObject("E"), OrbitalObject("D"), OrbitalObject("C"), OrbitalObject("B"), OrbitalObject("COM")],
    OrbitalObject("I"): [OrbitalObject("D"), OrbitalObject("C"), OrbitalObject("B"), OrbitalObject("COM")],
    OrbitalObject("COM"): [],
    OrbitalObject("D"): [OrbitalObject("C"), OrbitalObject("B"), OrbitalObject("COM")],
    OrbitalObject("G"): [OrbitalObject("B"), OrbitalObject("COM")],
    OrbitalObject("H"): [OrbitalObject("G"), OrbitalObject("B"), OrbitalObject("COM")],
    OrbitalObject("F"): [OrbitalObject("E"), OrbitalObject("D"), OrbitalObject("C"), OrbitalObject("B"), OrbitalObject("COM")]}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_simple_program_1() {
        let input = "COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L";
        let orbit_map = OrbitMap::parse(input);
        assert_eq!(orbit_map.number_of_orbits(), 42);
    }

    #[test]
    fn smoke_simple_program_2() {
        let input = "COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L
        K)YOU
        I)SAN";
        let orbit_map = OrbitMap::parse(input);
        assert_eq!(
            orbit_map.number_of_orbital_transfers_from_you_to_santa(),
            4
        );
    }

    #[test]
    fn test_advent_puzzle() {
        let input = "XV5)LZ5
        6JC)7ZG
        NCW)VDZ
        CPM)N4Z
        3PZ)4D6
        3TT)QPK
        G1T)FD5
        S43)3G7
        26J)ZMX
        GTS)G9S
        1GM)9ZN
        WCW)XTJ
        9B7)NTZ
        QDH)P9B
        FN2)6NB
        8XB)H8X
        638)2KM
        R42)M9X
        GX7)TGJ
        HB4)6SC
        9RP)62K
        W69)787
        T5M)H81
        M1C)KF5
        Z8D)F1Y
        F53)N8J
        BKJ)KYJ
        N9B)SHB
        QNJ)BST
        CD1)RVG
        TBD)Q68
        XL4)VSV
        Y84)L1C
        Q26)HGL
        JWN)42N
        MXF)XT6
        T5L)JCX
        2ZH)PQJ
        QJR)GWG
        KDY)5Y3
        WCS)BRL
        3K7)JX3
        CHC)MCZ
        WFT)GBX
        7NN)QVT
        JJY)RRG
        C9W)CS6
        CR2)J1Y
        GQR)X9Q
        WSX)3KX
        DCV)GQR
        1Z8)MC2
        XJP)WQC
        N1C)MY4
        J75)4CW
        Y35)RR1
        K3D)XWG
        VWV)LXN
        P18)RSW
        RX1)5P9
        DMP)XNR
        FNG)H83
        DVW)B2Q
        JJ4)BL4
        428)TLP
        N74)6TQ
        RSX)45D
        NM2)1JD
        RSX)2LL
        M9X)PWW
        W2J)DQ9
        DCW)WKP
        Q3B)FMY
        RPD)Y7X
        NZ6)4PV
        NM4)SGJ
        BBD)WWV
        XN6)9QQ
        WPZ)WJ9
        ST4)T72
        NDQ)XJ9
        P5G)G3N
        5VY)CNC
        GQG)SSB
        M7R)8QN
        NR8)DQJ
        KSW)Z8S
        42B)JBM
        BTV)985
        N7N)SGC
        GW9)6Q8
        73H)FSQ
        8GR)176
        RWL)XCH
        M3Z)RZ1
        1ZV)6BV
        6MW)TCW
        2HF)6JJ
        47B)8R6
        1TX)DSQ
        55J)GL3
        99Y)D27
        CD6)PCP
        VW8)JNV
        FNK)Q3B
        HWB)948
        3CT)XP3
        684)RK3
        TDG)4GH
        DZZ)NCW
        GR2)BJX
        N2M)9GR
        331)FGR
        HDQ)K74
        QC2)12V
        5TP)344
        QPN)4WD
        QJG)TWS
        15P)9NX
        4CV)WLD
        XGN)C88
        1Y6)7LB
        Z32)TNV
        G2V)QNQ
        673)YPW
        22J)628
        G46)7PK
        JN8)KZK
        4CW)PJS
        DMY)QXC
        7LB)L7S
        LQ9)X8N
        C3T)VWL
        BZ8)M2F
        XQ3)QKV
        89K)4YJ
        9BB)KV4
        X8N)1PJ
        PZT)98P
        C88)YSP
        LDV)T39
        1ZQ)SND
        TJJ)YKM
        7N4)8V8
        BBD)S66
        5MF)2X2
        GWW)LTH
        KQK)KPG
        FNV)HWD
        SMJ)HWZ
        M41)2W1
        W1G)C5D
        X79)P3C
        4KT)N5Q
        7BS)BWR
        CLX)DCW
        C1T)Q7C
        4K5)Y84
        63K)H8L
        PRZ)8JW
        LNM)8HG
        N4Z)GKB
        7DX)C1F
        BJX)LX3
        NWL)LK5
        LGW)ZXN
        XV7)W6H
        XBH)44C
        376)1LQ
        HQL)9BB
        PX2)3KS
        VHC)XP9
        4YZ)3XD
        SVN)2T8
        G8Z)ZP3
        YXP)H6D
        4C6)MLH
        BST)QV3
        RXP)CBM
        M9X)WCW
        5PJ)W3R
        98P)PXC
        MB3)TDG
        J4L)GJM
        6BV)59Y
        CNC)7P1
        Q7V)1H5
        3WF)65H
        MK1)SM4
        6JJ)YSV
        JFL)HC5
        6G7)1LD
        4GY)G9V
        B1Q)WCV
        3QY)K7W
        TSC)KDY
        SQS)1WB
        LV2)2SC
        FNV)4QX
        M2W)47Y
        Z8S)PQ9
        VSY)S88
        6TD)CP7
        DCD)4CT
        3NQ)R39
        6D5)YZC
        QFC)DDG
        XTJ)9SX
        SYH)Y8J
        KZ9)V3D
        32H)DD7
        W9V)YP3
        HQ1)RYK
        XHR)1TD
        7VD)4S5
        H81)PQR
        838)D9H
        JGT)R7V
        2MX)FTS
        1TD)CKB
        V2Z)F53
        7XL)563
        SM4)J3M
        DMP)Z9S
        J3G)6JC
        MWB)VPM
        NTS)V21
        2LG)97S
        9W9)CS9
        HLB)M2L
        WJ3)DTH
        G3S)5BD
        2YJ)PPH
        ZXK)HXX
        VFR)G3L
        1JK)L36
        TVV)4GY
        4HY)F1F
        JBM)XZ1
        BL4)X1D
        NF8)9KV
        SXX)PYS
        HPD)HBH
        RMF)HHZ
        4HK)5G8
        XZX)1ZV
        MCB)3YW
        LP4)JMQ
        WDL)JWB
        757)B3C
        MQX)8TY
        HMN)HV1
        J18)SKS
        LXJ)B41
        ZGV)JVM
        DC1)3K5
        QMC)SVQ
        VB9)2Z4
        WHT)BKM
        CDK)MS5
        SLQ)ZRZ
        7FL)GR2
        9T2)K81
        2T8)RX1
        DGR)PMG
        DGP)R3M
        NC8)WTV
        LK5)4XF
        132)QBZ
        D79)19N
        B4D)1FM
        53B)HLN
        HS1)RCM
        LQQ)CV3
        HJW)RB8
        LC7)BFM
        28Q)QLG
        CS9)G22
        Q98)GX7
        P9B)N1C
        JNR)VHW
        NVK)W4N
        V5R)HB4
        D89)5P6
        YQ3)LB9
        RSW)Q6W
        QM5)9XS
        TQM)3TT
        BV8)WCS
        9HZ)HMP
        4LX)XN6
        FGR)7Q1
        V9F)G1Z
        GBX)718
        2HR)SLL
        PZB)LGP
        PBC)32W
        5JZ)N3Y
        TZ9)2S1
        CNF)4YS
        T21)PKL
        4H2)XGN
        6SB)2HW
        R1V)LQB
        DCT)NKZ
        XTB)98Z
        CRZ)4LX
        8W7)JQQ
        C8M)92K
        3YW)YBY
        C76)42B
        R3M)FXF
        2VF)C12
        P1T)B4H
        ZP3)NM6
        KJB)JTN
        DTB)WMD
        RS8)2BX
        GCB)5HK
        4WD)F2T
        BZB)8NK
        NBK)SGD
        QV3)KXN
        WVG)4HY
        KRN)3VW
        M4Q)J75
        STH)T8Q
        GNV)5VY
        366)ZMV
        41V)PBC
        MHQ)3N4
        KVB)312
        K7Z)YYJ
        HBF)11M
        F2Q)17H
        2YZ)QV6
        THG)5WF
        S8B)V5H
        LY5)MQM
        TVK)FD6
        72F)QMQ
        LCT)HBZ
        LGZ)DLC
        RQ6)CPW
        V9J)Y5Y
        4LK)WN1
        F2Y)F8D
        WQR)RMZ
        Z42)JZ3
        QNQ)29B
        F1W)CFT
        T3J)13Q
        6NB)HVY
        G64)88J
        DD7)W3G
        CXY)CNF
        FD5)9C2
        8QH)FCS
        4K5)KZ9
        C12)NTS
        PJJ)298
        9G7)5BF
        HYT)GHC
        CVT)QKK
        PX2)LC7
        VN8)8DZ
        LT5)TYL
        12Z)RMY
        GRY)DRD
        G6R)RDJ
        864)ZT9
        DKD)N74
        L74)SWF
        2B6)4LK
        8V8)CLB
        45K)LL9
        YBY)JF4
        Q9Q)55M
        78F)6RM
        HYN)7TH
        7QN)HP9
        TB2)ZZM
        39D)T5W
        C5P)LGW
        DLD)RSX
        9RZ)HPC
        7TL)9T1
        Y3S)CRZ
        DTH)35N
        FD7)K3D
        YNK)FNB
        G1P)758
        68M)BQ8
        YMP)VW8
        PJZ)PJQ
        JJY)Y6B
        MD4)XNT
        4CS)SS5
        2X2)B52
        2TR)XL4
        H8X)C4N
        SV5)TL5
        CGN)QM5
        MXZ)GSX
        Z8K)JT2
        738)MR5
        2ZS)4F6
        QSD)NRM
        Q48)N1T
        4SH)99Y
        3VW)M3Z
        62K)HYN
        XTQ)9VX
        1FS)KF2
        9VL)K71
        XVB)7FL
        QHN)DSL
        Y6B)KRC
        SNJ)X8C
        ZDL)DPL
        2TR)3FH
        CP7)TG1
        3K7)L7K
        KQK)MK1
        53H)MY9
        7FW)S5B
        QL9)LF8
        4FG)MJY
        FMY)6VK
        G3Z)HDQ
        T39)BD9
        R6B)YFS
        HCX)72F
        QKP)RF9
        N1C)462
        9RM)B1Q
        TLP)7QN
        QSW)Q72
        NH3)253
        4TG)DCT
        Z4K)GV9
        4YS)689
        KSQ)P2S
        SV7)Z81
        D2J)73K
        RSY)FG7
        5PR)66Q
        QY8)HYT
        Z52)VBB
        3P3)7CQ
        GWD)3PZ
        GH6)RRF
        YQ2)YSL
        TQM)F6N
        TWT)R8N
        YYJ)X86
        LDV)362
        TV5)S99
        JTN)VHC
        42N)BX1
        LMV)XQ3
        Z27)Y1V
        DWK)RTT
        64J)3CT
        VWV)RF3
        NLV)1L3
        M4Y)JB3
        JGL)LR3
        F4T)LK7
        CD6)CR2
        167)7LW
        ZMV)4D3
        ZCY)X62
        G3N)VSY
        3SC)W6C
        N3Y)4KT
        V3D)HRV
        QGH)KLP
        XV7)JZ8
        YR4)CBG
        L6B)N1V
        628)STY
        YQX)FLG
        4RR)NKX
        L7K)SVN
        VBQ)SLK
        H3Z)BGN
        5P6)NY3
        G7X)2NG
        M1C)7JS
        658)2HR
        QVN)4HK
        6D4)NGS
        XNV)GH6
        DKJ)1CT
        PCP)JSF
        DRN)YOU
        88J)QVN
        Q4W)658
        LGP)FQG
        SWF)DYZ
        DRD)PSB
        G8Z)HTG
        LVD)HWB
        D3D)63H
        S94)FKM
        26M)RQ6
        Y7X)89R
        CPY)BZB
        4WQ)1ZQ
        XNR)XGT
        221)8W7
        T53)KP7
        3K3)RLH
        FR7)8ZB
        QG8)KRN
        G1Z)2ZS
        GD9)455
        9SX)3NC
        WTV)9T2
        D61)F1L
        VSJ)2SB
        M8D)7VD
        CV3)TVK
        TVX)VJV
        7P1)28Q
        69P)DLZ
        VYR)83M
        VQS)3Y8
        W34)LV2
        Z9S)6KV
        PFV)GHF
        YKB)CXY
        8NK)GL1
        NWF)KJB
        SXP)JNL
        JCY)DFT
        SKS)W7D
        P19)BJC
        MW3)WPW
        SPV)HVT
        S5T)KVN
        3HW)C92
        6HZ)W74
        1JD)K41
        CFT)WSS
        61T)27F
        4GH)Z8K
        D27)7H9
        78F)TV5
        HFX)NWP
        RHS)DQL
        819)4V3
        RGW)738
        Q4Y)T21
        RRF)248
        MK7)D9X
        55F)QFS
        8M8)NM2
        J4X)LFH
        B1J)T6K
        4F6)PZT
        WN1)9WR
        VXM)3K7
        7RC)NW1
        TF2)9CM
        59Y)6WQ
        G7T)BF5
        Z4Q)JKJ
        ZCD)LT5
        2QP)MRX
        6TM)D79
        14T)KBG
        RHS)9KD
        7JS)Z4Q
        XVN)B1D
        Y7X)17K
        DRD)Q9Q
        3MH)BXK
        TCW)9TC
        9R4)NBY
        7ZG)B4M
        PPH)3LC
        9KD)VRQ
        91T)KSX
        FS2)3S9
        2WF)TWT
        PQJ)FQT
        8D8)ZGV
        1ZV)FN2
        4TB)LDQ
        JDY)6N8
        35N)756
        WPZ)V5R
        HYH)QK8
        JCS)JL9
        1Z8)MPR
        JPN)8W4
        34C)85G
        FNG)F35
        B3T)819
        PLQ)138
        VBJ)JTC
        JB3)FS1
        62X)LM8
        FZP)4SH
        1P7)LNM
        JZP)56P
        1QR)SQS
        VRQ)LMV
        WZG)7B1
        KPG)N3V
        4C6)VQS
        JQQ)MDR
        YCZ)69P
        RB8)9H5
        23R)9JW
        7XH)WRC
        JZ3)TYF
        5SG)1CH
        JNV)HCX
        312)QJG
        4T1)NS1
        64L)N1L
        6NK)LC1
        4WQ)FN6
        BKF)68D
        GB4)K95
        HX7)JPN
        W2M)6QS
        JSG)91T
        TP2)CWM
        NP1)S94
        3Y8)KWK
        2NM)ZJ5
        CBG)JC6
        KJB)ZPJ
        XPM)TF2
        BR7)L7C
        SHB)PY2
        PMG)F33
        2F1)Z36
        SS5)9K7
        73F)PL9
        6TH)9LP
        9MY)WHF
        63W)Q98
        YSP)DY7
        4WD)Z3C
        VCV)FNF
        QSW)PJZ
        PV4)9VL
        NY1)H37
        D81)NRC
        XYM)GP4
        JX4)LKF
        K81)FG6
        HMD)W34
        DYZ)8KY
        DY7)6WF
        8GC)ZDL
        WSS)X88
        DVV)R1B
        BB7)1DN
        3NC)7S4
        C12)TNS
        L2H)14T
        Y3P)92P
        W58)44W
        MCB)YPZ
        XQZ)DJH
        KQZ)MN2
        9WR)KCD
        2JM)D5L
        5Z7)9B7
        QKY)MV1
        K41)QKY
        VTF)BQ9
        NDZ)QNJ
        BGN)PJJ
        P77)6YV
        HT2)YWK
        PPJ)CQX
        H83)QSD
        VSV)Z8D
        LXN)QG8
        R87)3DM
        W6H)ZGN
        MKN)GD9
        3J7)1V9
        7FW)GQZ
        63H)WV5
        1PJ)5HM
        S88)SQ6
        XCH)PXL
        MSH)ZMH
        RR1)XJ8
        NTH)67R
        BWD)F1S
        JF8)9G7
        DYL)TNX
        W34)NVK
        8K3)53B
        75S)DVW
        4QX)3DS
        RSW)HQ1
        3HS)F2Q
        B4H)DX6
        6JX)HQL
        SV5)ZFX
        KJ1)WJ6
        89R)84D
        FJB)QYY
        K5S)55J
        F3G)WY8
        1CP)JTF
        WT9)LF9
        9TC)N8L
        FBP)R5D
        57B)D61
        XSG)3WX
        675)WHT
        B33)VYR
        K2H)3PN
        8ZB)SLB
        C1F)LLG
        3PN)5FH
        QKP)HL9
        G9S)3QY
        ZQF)F4T
        FXM)517
        L74)2HK
        V31)QJ3
        DDG)PY6
        3YW)6XG
        CXD)TBD
        K8W)J6H
        BX1)VS9
        LVJ)331
        B3J)4TY
        3S9)FRD
        CZ9)QD7
        718)SMJ
        53G)S8B
        LR3)W2M
        FRG)32P
        VBB)23R
        JM7)JY4
        3GM)HLB
        8HG)BCY
        CLV)WKS
        CFB)MW3
        8KY)FQM
        3N4)SD9
        C3T)DZZ
        75P)Z8Q
        MP2)VTS
        CW7)PZD
        GSV)TJJ
        5G8)WPZ
        45D)RGW
        VXC)2YG
        GLD)Y35
        L7C)P18
        PSB)PFS
        QPM)6R2
        1NB)BKF
        787)754
        KRC)B4D
        N5Q)DGP
        718)B5P
        44C)ST4
        XVY)45K
        WPB)VSB
        PKL)1WV
        7DG)37F
        QF5)QRB
        QC2)MK7
        KPZ)3SC
        8M2)LXX
        VPZ)45M
        V5M)34C
        JMQ)ZB6
        B4M)G18
        D6Y)DYT
        LFD)B9S
        R84)72H
        XP4)L74
        H37)BDG
        ZTS)NBK
        QKC)G3K
        7W1)FV4
        16P)M37
        ZSC)XJL
        B1D)VBJ
        RW8)5X5
        4D2)54L
        212)WZG
        NY3)TB2
        LR3)6D4
        KYJ)Z52
        9CM)98M
        RDB)LHB
        W48)V9J
        69X)ZFL
        45M)NV5
        54Q)QK1
        P18)B38
        QPD)1WG
        4DX)9HZ
        JC6)598
        LHT)8XB
        WKY)VMF
        RDV)PMX
        XLJ)LRG
        MPR)N15
        C9Z)FKC
        LZ5)C2K
        MW5)TFF
        FV4)9BW
        2RJ)K8W
        9GR)18Y
        1LD)JGG
        82Z)HKD
        7JS)4HT
        7NC)CXB
        2VZ)YL1
        PQR)JWN
        HZP)SV5
        212)42H
        QGD)J6P
        D5L)6NK
        XP7)6TM
        18Y)D48
        4TX)4NG
        NZJ)ZSC
        WGD)QWY
        NX4)66X
        2MG)BCF
        PSV)FVJ
        GD7)QC2
        DLZ)542
        2W1)DRN
        1WB)P6H
        KTF)H5W
        Y8J)9ZH
        JWB)PYW
        1DN)ZG6
        4MX)TSC
        5BF)PYB
        SLL)KSW
        RGR)4CS
        9T1)T26
        7H9)3SR
        BRP)TD9
        B73)C5P
        BXK)VN8
        4KT)6X5
        B65)BTV
        N88)2V5
        P2F)6SB
        PYW)QTV
        MJY)684
        JSF)P77
        LP4)P4J
        LKF)RNH
        SSB)3VB
        D75)F2D
        SD9)HDZ
        S46)1L7
        QYY)638
        XJJ)HJT
        R5D)Y72
        TXH)VN5
        WLD)HT2
        DLC)36P
        WS3)XLF
        HKD)V9F
        CKB)2YZ
        57X)RX9
        YKM)DTB
        4PR)G27
        T5W)55F
        PZD)JGT
        4CT)8FR
        HL9)W5B
        HWD)MW5
        XT6)2ZH
        BQH)9C8
        MY9)HV8
        X5S)RDN
        VSB)16C
        X86)3HW
        84D)26M
        MS5)M1C
        6X5)NWF
        9XS)L8X
        8R5)DXZ
        3KX)6B7
        S99)VWV
        5PV)D89
        FTS)8GR
        STY)9QL
        Z4L)6KM
        ZN3)Z27
        9G5)9F4
        7YP)167
        KF2)PX2
        72L)M4Y
        BSL)424
        Q7C)M4Q
        9GP)FJB
        SG6)NP1
        SVQ)QKC
        J3D)X5P
        KFR)RR8
        216)HMD
        KXN)93W
        Y8V)QGD
        Y8V)PV4
        248)LVD
        WFP)G1T
        NKZ)SNJ
        CX5)PSV
        QWD)THG
        J6P)C1T
        HV8)PMQ
        SGJ)MD4
        K3D)4Q2
        6R2)Z32
        KH9)72L
        9QQ)JR3
        B9P)4T7
        RNH)154
        6HQ)BRP
        HP9)WPB
        HGG)7DG
        27F)VSJ
        LW1)NWL
        GD7)LW1
        LWN)6TH
        9L1)TLR
        V7Q)3G5
        5P9)J3D
        B3N)XQT
        S59)5QP
        JR3)BW2
        2KM)R2V
        6KM)FNG
        6Q8)H2B
        CHD)RRQ
        4S5)7FY
        289)MWB
        73H)Y3P
        HYW)TXH
        X6L)XZX
        6YV)2YJ
        ZFL)4T6
        MM1)JX4
        63K)6D5
        P8G)Z9J
        NBY)Z4L
        DCT)LPH
        STQ)ZPT
        MCZ)VPZ
        KSX)YNK
        T6K)26J
        WWV)216
        LJW)Y3S
        W7D)VWC
        4SH)8MM
        NBY)6TD
        5HM)KQ9
        XP9)MQX
        YSL)3Y4
        NL2)DYL
        19L)8K3
        RZ1)9VC
        QVX)WVR
        9F4)Q3F
        2K5)N4X
        RF3)NHN
        WRC)G1G
        P8R)7TL
        XJ8)K6F
        5CZ)281
        B2Q)DG5
        5YC)XK3
        92K)DWK
        9ZH)YTR
        VWC)FQP
        BFM)B65
        Z9F)PDP
        8FR)6G7
        RDN)CHD
        XP3)RGR
        WVR)MMB
        7LB)5MB
        789)XCW
        V3L)19X
        689)2NM
        7DR)GLL
        RDV)Z4K
        HBH)F2Y
        HW4)B4W
        Z81)P8G
        XZ1)VYN
        T26)KDJ
        RCM)P2F
        F1Y)TCT
        T8D)V2B
        HV1)QPD
        KL3)CRS
        WY8)R42
        9VX)675
        6KN)LQQ
        LHF)NY1
        98Z)2VN
        3KG)JZP
        HHZ)2T6
        Q44)5JZ
        4HT)QF5
        9C2)FH4
        KFR)W9V
        L7G)4Y2
        GHC)L2H
        26J)YR4
        D9Z)XP4
        CRS)8MG
        J9B)P91
        J1C)8KQ
        CCR)M6M
        B52)PQC
        HLN)RSY
        QHM)G64
        BWR)47B
        G1G)8GC
        X6L)3TF
        FSG)7NN
        LF8)7FW
        WMD)WQQ
        45K)2MG
        MMB)JJ4
        3DS)XVN
        M2L)2X4
        ZYF)4JR
        MV2)Y4C
        PDP)JGN
        GWG)M2W
        XQT)DC1
        6QS)C28
        KRT)DMY
        NS1)L5X
        FY4)SK4
        ZPJ)HWH
        R39)PPQ
        J67)KNT
        DZ6)W48
        626)DGK
        4T7)VLZ
        HTJ)GLD
        PXL)PRZ
        DCW)WHP
        TDG)RDV
        GF6)F7V
        49C)DZ6
        3YR)5PR
        M8N)64L
        WHK)62X
        1WV)788
        ZPT)4MX
        NV5)58L
        9WY)8K9
        ZT9)D6Y
        MKN)S43
        Q65)Z73
        QPL)XVY
        X7B)NDQ
        153)BDX
        R2V)395
        YWK)N34
        ZMX)GX4
        3DJ)X11
        RHM)2VZ
        P3N)366
        ZRZ)6JX
        559)662
        WZ4)C8M
        KC4)VQC
        FS1)B3J
        72H)YWP
        H8L)TP2
        QK9)XXQ
        PL6)6PM
        32W)P8R
        YSV)4TX
        PY2)VD5
        LHB)4PR
        R1B)J4L
        36B)9NC
        FXF)4K5
        QPK)C3T
        VYN)VSW
        4NG)J65
        BZ8)LWN
        P5G)HJW
        2Z7)BX4
        BSK)16V
        S7S)9YQ
        TNS)5SG
        B3C)QK9
        KQ9)9L1
        VLZ)7Q7
        4PP)F1W
        2NG)F8R
        JH3)63W
        87R)KMF
        YPW)YGC
        F1L)BBD
        QLG)NH3
        S1H)K7Z
        F2D)DN4
        WJ9)BMS
        P7C)5YC
        428)HBF
        2HK)75P
        7S4)RT8
        D48)PL4
        B73)B5R
        9L7)1JK
        3G5)P7C
        B8P)BB7
        6G7)K2H
        598)V3L
        QBZ)BR7
        GQ5)HYW
        RLH)DCD
        BD9)CMV
        N34)J18
        3V7)KFR
        6N8)4J3
        J6H)QHN
        JGG)F6G
        WJ6)X5S
        KSJ)D81
        662)VCV
        4XT)B1J
        JWB)SXX
        XN6)XLJ
        LFH)TFP
        RT8)Y5D
        QKY)LY5
        PYB)FD7
        F1F)WDL
        CWM)RL5
        RR8)C8B
        18B)PWN
        F35)3HS
        B9S)7D4
        5BD)X79
        TGJ)9HF
        NNJ)235
        DTH)NDS
        GLL)CPY
        138)RXP
        X9Q)LHF
        WNV)DF5
        19N)46N
        QKV)BYV
        7TH)JNR
        GL3)288
        66Q)YYR
        SK4)4VH
        QNJ)XQZ
        7CC)278
        BDG)T8D
        WBM)YMW
        NCW)RHM
        2V7)YGW
        PYB)XSG
        RX9)YMP
        BCY)J1N
        JZ8)5CZ
        362)QVX
        517)QPM
        K74)3WF
        G9V)MHQ
        Z8Q)CJ2
        69M)54X
        STB)CHC
        VSW)GWD
        CMV)XTB
        17H)XNF
        CXG)HNZ
        HNR)LVJ
        S5B)QJR
        Z73)6MM
        WQT)7QR
        P6H)VHY
        BZ4)B33
        NM6)DKD
        9KV)419
        235)G2V
        K71)6MW
        T8Q)Y62
        68B)Q3S
        8MG)68B
        FSQ)3GM
        4XF)WJD
        FG6)36B
        DYZ)XDF
        BX4)XV7
        77F)WQR
        JTC)HTJ
        YZC)MVQ
        LM8)2B6
        ZB6)SXP
        PXC)5TP
        F6G)5FM
        G18)4WQ
        R4M)HFX
        HMP)2QP
        Q1L)2RJ
        QRB)9G5
        YJZ)STB
        3FH)3K3
        356)DKJ
        5GR)27Y
        BNQ)NC3
        36P)G7T
        YN2)HYX
        P76)39D
        PJS)YQX
        C76)C1P
        27Y)65Z
        HVY)37H
        462)16P
        N8L)Q44
        FQM)BWD
        54L)DQH
        455)P1T
        5GF)6JL
        JWR)WFT
        FG7)H2M
        DBT)J9B
        Q72)6KN
        756)BNQ
        MQG)HNR
        HDV)VGH
        FKM)MXF
        6PM)V7Q
        HPC)QWD
        G5M)Z1T
        FRD)8ZS
        ZPH)7TK
        KV4)NLS
        7QR)7NF
        RL5)JH3
        CPS)1HR
        BDX)B8P
        ZB6)XVB
        CS6)YXP
        W6C)C42
        STY)ZXK
        WXD)2TR
        KNT)J76
        XQ8)GQG
        1HR)D75
        MLH)56K
        PFS)X2B
        FVB)W7K
        6YQ)82Z
        RK3)XFT
        NRM)SG6
        9QL)VTF
        9TC)61T
        6RM)5GG
        8MM)H3Z
        SGC)KZ3
        7Q1)Z71
        HGL)FXD
        B69)4XT
        N1L)YQ2
        QV6)YCH
        K3N)KNV
        68D)BSL
        Y4C)HPL
        VDZ)WXD
        C5B)6SR
        PMQ)NVZ
        M35)NSK
        B4W)QCQ
        6FC)1GM
        7TS)N9B
        FN6)BKJ
        9H5)QL9
        KDJ)DCV
        LPH)W1G
        BQ9)SD1
        4Q2)KSQ
        6WF)4YZ
        M1W)5GF
        HYT)WVG
        HDZ)VBQ
        DZ6)VSS
        YTR)428
        T72)R8V
        F1S)C8R
        1FD)4ZK
        41J)626
        658)6CN
        HZ6)FNK
        FKM)1Z8
        6KV)NZJ
        F33)DJ1
        XTB)TSF
        NXV)RYS
        F1F)5MF
        YGW)2Z7
        MC2)CCR
        3YR)GSV
        XQK)XP7
        M97)PL6
        1CP)9ZP
        N8J)C9W
        GQZ)L6B
        HV1)SYH
        9VC)VZP
        QD7)5GC
        X2B)VT1
        92P)T5L
        XCW)WKY
        MY4)3J7
        MGN)3VL
        K7W)8R5
        HWH)BT3
        4PV)9R4
        FVJ)5BJ
        5MB)18B
        F8R)7RC
        MGN)N88
        1CH)ZPC
        X88)YN2
        Y8J)7C2
        1H5)R53
        BMJ)SV7
        LLG)G7X
        9LP)M97
        ZDL)B6H
        5WZ)MV2
        RVG)442
        W3R)CCX
        73K)L33
        17K)HGG
        LC7)GNV
        Q27)Q1L
        2HC)V2Z
        HXX)KC4
        6WQ)JCL
        T92)M8D
        KNV)6FC
        T9B)559
        RKY)DBT
        RRG)LRM
        DSQ)PCQ
        KF5)6YQ
        R8N)FXM
        JL9)KQK
        RCK)3P3
        8W4)XBP
        TNL)XYM
        4NG)Q26
        JX3)Q27
        GX4)8QH
        GTC)GD7
        LMV)XQK
        YZ8)2MX
        YL1)PFV
        VHW)WT9
        S99)58S
        13Q)NDZ
        LHB)LHT
        FQT)S59
        DX6)RWL
        288)MP2
        2LL)4H2
        ZPC)GTC
        HLN)4RR
        22J)9MY
        FH4)19L
        NF3)KRT
        JCL)DGR
        GSX)FSG
        TG1)X7B
        NHN)S46
        FZP)P5G
        S66)XTQ
        WQC)G1P
        XNT)QSX
        V14)673
        TLR)B9P
        2YP)ZPH
        FKC)VFR
        56K)QHM
        B9W)133
        TNV)P2L
        9NX)F98
        8KQ)5VP
        5PH)FY4
        6VK)S1H
        ZZM)M7R
        PLQ)MXZ
        GFY)RW8
        758)78F
        FQ1)JCY
        WV5)W58
        F3G)XPM
        GLX)GL5
        LQB)7BS
        Z55)GQ5
        THN)KH9
        5GG)V31
        G1P)FR3
        XFC)K5S
        HQ5)XBH
        HJT)FRG
        VSJ)MB3
        VHY)FQ1
        VL1)RDB
        L33)1NB
        M6M)LXJ
        BCF)J67
        6WT)B69
        FQT)R6B
        L3S)HS1
        VSS)C5B
        9FD)G5M
        P2L)GCB
        L5X)1VY
        26X)5Z7
        4D6)LWG
        8DZ)WT6
        XWG)V72
        QJ7)M1Z
        58S)HMN
        HJR)QY8
        L7S)ZYF
        MR5)5PV
        C8B)WFP
        QK8)1TX
        J12)2V7
        J76)RS8
        6MV)WJ3
        16C)HLY
        4TY)73H
        H6D)XQ8
        4JR)MX5
        M37)Q65
        W74)XX9
        TK5)49S
        B6H)BZ4
        F6N)BKV
        JF8)ZTS
        WBS)4LS
        LTH)J42
        JNL)6WT
        GL1)5Q3
        XJL)KVB
        YMW)MM1
        9HF)356
        5QP)7XH
        K95)KJ1
        QWY)8W6
        KPN)J1C
        RRQ)XNV
        419)XMQ
        YYR)9RZ
        1CT)KLZ
        8XQ)V5M
        XGT)ZN3
        WZ4)G8Z
        YWP)3MS
        93W)18V
        7FY)M8N
        TFP)NM4
        FR3)7N4
        JGN)NF3
        JDY)1P7
        M17)WS3
        SLQ)RVW
        DQJ)XR9
        FNF)89K
        YKH)KKH
        K6F)RPD
        J75)9GP
        2X9)DLD
        6CN)7W1
        7TK)MGN
        2SC)3KG
        BMS)MQG
        ZFX)53H
        Z3C)LT3
        VDN)GRY
        6JL)9Z9
        7NF)PML
        14T)WSX
        1V9)N7N
        N3V)HPD
        F53)PPJ
        DQ9)TNL
        JPH)1FL
        L1C)WBS
        RMY)BMJ
        MV1)1CP
        4GY)8M8
        CGN)4GC
        7B1)26V
        8QN)JSG
        VQC)XRT
        J1N)MKN
        PYS)289
        7Z9)HDV
        SJB)4C6
        LX3)HX7
        94G)R1P
        DY7)D6W
        3FH)2RV
        XKS)WHK
        FNB)6L2
        K6Y)NK7
        BYV)Z9F
        QBZ)FVB
        C5B)73F
        6SC)DMP
        B5R)K3N
        BKV)YKB
        VW8)1FD
        PWN)QPL
        DG5)BQH
        1MD)YCZ
        KWK)CZ9
        JY5)NX4
        F2T)8QR
        RYS)KF3
        LRM)M1W
        LXX)CD1
        98M)J4X
        D6Y)M17
        97S)BSK
        X11)2X9
        WKP)57B
        CCX)221
        GJM)T5M
        LT9)7YP
        RVW)FZP
        62K)CLV
        3VL)QQK
        3XD)HQ5
        W3R)QFC
        8JW)32H
        KCD)2HC
        SND)9FD
        4PR)D3D
        Q68)77F
        56P)87R
        SD1)RHS
        COM)2WF
        Q53)HBT
        ZJ5)68M
        4VH)VDN
        Z36)2QN
        FD6)R96
        NWP)HZP
        4T6)LGZ
        QTV)GF6
        S2P)HZ6
        XFC)S8R
        2V5)QDH
        PPQ)CD6
        Z71)YKH
        RYK)864
        VMF)MZQ
        KZK)KTF
        KZK)XKS
        SLK)WBM
        HWZ)CX5
        N1L)2HF
        54X)JWR
        TYF)YY5
        3DM)264
        FLG)FBP
        D2J)376
        2RV)JPH
        PMX)12Z
        CLB)7TS
        CNC)RCK
        281)8M2
        2SB)R4M
        KF3)1RN
        B38)4FG
        GV9)XV5
        33M)676
        QGN)HS2
        7D4)GFY
        4CT)STK
        N15)R1V
        29B)Q4W
        DXZ)BV8
        563)GLX
        XDF)GRS
        1LQ)TK5
        BPW)C93
        KP7)GLH
        2X2)F24
        YTR)LT9
        MN2)NNP
        W7K)TF6
        DGK)KPN
        D6W)PVB
        67R)3DJ
        TB6)NLV
        MDR)PLQ
        C92)3NQ
        F8D)1P3
        4ZK)T9B
        347)JJY
        XNF)6HQ
        QKK)63K
        B4H)HYH
        TCT)GFD
        CPW)5MK
        BQ8)789
        SGD)R84
        CBM)7XL
        NKX)4TB
        9K7)GTS
        VWL)3V7
        M1Z)7DX
        XXQ)W69
        676)Z55
        1NP)GW9
        HYX)Q7V
        YY5)5WZ
        7PK)Q12
        2VN)GWW
        NC3)SXB
        H5W)V14
        PQC)CPM
        JVM)RZL
        WQQ)R87
        YQ2)JCS
        3SR)MF9
        RPG)BZ8
        L36)CDK
        6WT)KSJ
        QSX)9RM
        STK)JM7
        DPL)CXG
        8DZ)FNV
        R96)7NC
        C5D)SAN
        1FM)4T1
        FXD)KCT
        K7W)347
        C1P)FS2
        C8R)Y4D
        F35)Q4Y
        3Y4)Q53
        WHF)212
        RMZ)TVV
        DKM)Y8V
        395)C76
        65Z)JF8
        4LS)CFB
        BJC)132
        P4J)WDH
        QQK)LCT
        TWS)LP4
        Q6W)XJJ
        GRS)JDY
        1WG)G46
        PCQ)X9P
        12Z)41J
        JF4)LXM
        F98)2LG
        DQH)15P
        DTB)9L7
        6XG)8D8
        TKW)2JM
        X5P)M35
        154)CGN
        7LW)KRQ
        KCT)57X
        9BW)QGN
        GHF)D2J
        Y72)HW4
        4V3)LFD
        ZXN)T3J
        W5B)L7G
        NP1)JGL
        6L2)J3G
        6B7)NTH
        DFT)DKM
        GP4)HXH
        FQP)THN
        XQ3)QPN
        H2M)ZCD
        37H)1RM
        MQM)BDT
        Y5Y)PZB
        WT6)B3N
        R7V)MSH
        2T8)T53
        3WX)WZ4
        WHP)69M
        YFS)NF8
        G3K)4TG
        V2B)3YR
        5HM)CLX
        5HK)75S
        HC5)SLQ
        55M)KPZ
        KKH)4CV
        B65)SJB
        TFF)1Y6
        5Q3)HJR
        MVQ)MCB
        12V)757
        YQX)V73
        9C8)JN8
        DYT)5PH
        C8M)69X
        VGH)S5T
        TNX)CXD
        XK3)TVX
        1L3)7RX
        662)B3T
        RTT)KQZ
        21H)RMF
        ZQF)2YP
        8ZS)7DR
        X8C)GY8
        HNZ)JPZ
        133)1NP
        PJJ)CPS
        18V)RPG
        9NC)STQ
        J1Y)3MH
        6SR)B73
        HPL)41V
        YMW)BPW
        RDJ)KN3
        LXM)P76
        42H)9W9
        CCD)W8T
        176)S7S
        B41)G6R
        KLZ)P19
        SQ6)XFC
        TF6)H89
        2FV)8DB
        4Y2)4D2
        JTF)TKW
        LDQ)X6L
        LT3)T3K
        QK1)GB4
        W4N)7Z9
        BKM)7CC
        65H)S2P
        8K9)YJZ
        253)9WY
        KVN)4PP
        2Z4)F3G
        C93)KTW
        5FM)QSW
        F24)4DX
        T3K)NR8
        3G7)RKY
        V73)6MV
        BT3)2F1
        HLY)CCD
        GLH)94G
        2CZ)W2J
        NSK)NC8
        V5H)M41
        19X)NXV
        QMQ)54Q
        LRG)Q48
        7CQ)YB2
        TSF)8XQ
        FCS)2VF
        16V)VXC
        HBZ)WGD
        VJV)JY5
        XRT)J2X
        424)5GR
        SXB)XJP
        BJC)5PJ
        SND)49C
        LC1)8Y7
        FR7)L6J
        H2M)XHR
        BW2)C9Z
        2HW)185
        TYL)2K5
        15P)6HZ
        LK7)YZ8
        JPZ)NNJ
        NGS)G3S
        GL5)QVR
        DJH)NZ6
        GKB)9S3
        JCX)Z42
        P2S)T92
        9ZN)ZCY
        PY6)DVV
        YP3)W5Z
        11M)WQT
        PQ9)G3Z
        2S1)SPV
        CQX)ZQF
        YPZ)LQ9
        VRQ)TZ9
        GY8)26X
        985)YQ3
        9ZP)9RP
        9JW)VL1
        4MX)33M
        L6J)L3S
        KBG)P3N
        G22)RPQ
        948)1FS
        3KS)CW7
        WCV)WNV
        LB9)KL3
        BDT)21H
        Q53)VB9
        HCV)2CZ
        7RX)TQM
        4GC)LDV
        PL9)N2M
        1VY)NL2
        R96)153
        37F)53G
        VT1)KF4
        61T)838
        HBT)R4N
        H81)TB6
        1FL)FR7
        J65)D9Z
        8Y7)22J
        KZ3)JFL
        W5B)B9W
        G27)QJ7
        V72)1MD
        5G8)J12
        T39)STH
        6JL)HCV
        SK4)K6Y
        Z1T)2FV
        PPJ)LJW
        JKJ)64J
        47Y)QGH
        N1V)CVT
        PJQ)TWQ
        26V)QMC
        5Y3)1QR
        F1W)VXM
        49S)QKP";
        let orbit_map = OrbitMap::parse(input);
        println!("{}", orbit_map.number_of_orbital_transfers_from_you_to_santa());
    }
}
