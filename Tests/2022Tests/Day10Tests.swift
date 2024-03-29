import Foundation
import XCTest

@testable import Advent2022

class Day10Tests: XCTestCase {
    func testProblems() async throws {
        let day = try await Year2022().day(for: 10)
        let partOne = await day.partOne()
        XCTAssertEqual(partOne, "11820")
        let partTwo = await day.partTwo()
        XCTAssertEqual(partTwo, "EPJBRKAH")
    }
}
