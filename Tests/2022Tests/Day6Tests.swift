import Foundation
import XCTest

@testable import Advent2022

class Day6Tests: XCTestCase {
    func testProblems() async throws {
        let day = try await Year2022().day(for: 6)
        let partOne = await day.partOne()
        XCTAssertEqual(partOne, "1655")
        let partTwo = await day.partTwo()
        XCTAssertEqual(partTwo, "2665")
    }
}
