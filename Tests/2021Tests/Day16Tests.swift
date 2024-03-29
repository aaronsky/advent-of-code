import Foundation
import XCTest

@testable import Advent2021

class Day16Tests: XCTestCase {
    func testProblems() async throws {
        let day = try await Year2021().day(for: 16)
        let partOne = await day.partOne()
        XCTAssertEqual(Int(partOne), 953)
        let partTwo = await day.partTwo()
        XCTAssertEqual(Int(partTwo), 246_225_449_979)
    }
}
