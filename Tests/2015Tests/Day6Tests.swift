import Foundation
import XCTest

@testable import Advent2015

class Day6Tests: XCTestCase {
    func testProblems() async throws {
        let day = try await Year2015().day(for: 6)
        let partOne = await day.partOne()
        XCTAssertEqual(Int(partOne), 569999)
        let partTwo = await day.partTwo()
        XCTAssertEqual(Int(partTwo), 17_836_115)
    }
}
