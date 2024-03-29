import Base

struct Day2: Day {
    var presents: [Present]

    init(
        _ input: Input
    ) throws {
        presents = input.decodeMany(separatedBy: "\n")
    }

    func partOne() async -> String {
        let surfaceArea =
            presents
            .sum(of: \.surfaceArea)

        return "\(surfaceArea)"
    }

    func partTwo() async -> String {
        let ribbonLength =
            presents
            .sum(of: \.ribbonLength)

        return "\(ribbonLength)"
    }

    struct Present: RawRepresentable {
        var width: Int
        var height: Int
        var length: Int

        var rawValue: String {
            "\(length)x\(width)x\(height)"
        }

        var surfaceArea: Int {
            let lw = length * width
            let wh = width * height
            let hl = height * length
            let smallestSide = min(lw, wh, hl)

            return 2 * lw + 2 * wh + 2 * hl + smallestSide
        }

        var ribbonLength: Int {
            let lw = 2 * length + 2 * width
            let wh = 2 * width + 2 * height
            let hl = 2 * height + 2 * length

            let smallestSide = min(lw, wh, hl)
            let volume = length * width * height

            return smallestSide + volume
        }

        init(
            width: Int,
            height: Int,
            length: Int
        ) {
            self.width = width
            self.height = height
            self.length = length
        }

        init?(
            rawValue: String
        ) {
            let dimensions =
                rawValue
                .components(separatedBy: "x")
                .prefix(3)
                .compactMap(Int.init)

            let (length, width, height) = (dimensions[0], dimensions[1], dimensions[2])

            self.init(width: width, height: height, length: length)
        }
    }
}
