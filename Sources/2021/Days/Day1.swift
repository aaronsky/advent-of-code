//
//  Day1.swift
//  
//
//  Created by Aaron Sky on 11/16/21.
//

import Algorithms
import Base

struct Day1: Day {
    var depths: [Int]

    init(_ input: Input) throws {
        depths = input.decodeMany(separatedBy: "\n", transform: Int.init)
    }

    func partOne() async -> String {
        let increases = depths
            .windows(ofCount: 2)
            .count(where: { ($0.last ?? -1) > ($0.first ?? 0) })

        return "\(increases)"
    }

    func partTwo() async -> String {
        let increases = depths
            .windows(ofCount: 3)
            .map(\.sum)
            .windows(ofCount: 2)
            .count(where: { ($0.last ?? -1) > ($0.first ?? 0) })

        return "\(increases)"
    }
}