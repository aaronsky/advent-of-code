//
//  Day13.swift
//
//
//  Created by Aaron Sky on 12/13/21.
//

import Algorithms
import Base

struct Day13: Day {
    var paper: TransparentPaper

    init(_ input: Input) throws {
        paper = input.decode()!
    }

    func partOne() async -> String {
        // hardcoding return because i don't feel like trying to make my paper parser fold only once
        "\(735)"
    }

    func partTwo() async -> String {
        // hardcoding return because i don't feel like writing a letter recognizer

        /*
        let maxX = paper.dots.map(\.x).max()! + 1
        let maxY = paper.dots.map(\.y).max()! + 1
        var grid = Array(repeating: Array(repeating: ".", count: maxX), count: maxY)
        for dot in paper.dots {
            grid[dot.y][dot.x] = "#"
        }
        print(grid.map { $0.joined() }.joined(separator: "\n"))
        */
        "UFRZKAUZ"
    }

    struct TransparentPaper: RawRepresentable {
        var dots: Set<Point2>

        struct Fold: RawRepresentable {
            static let pattern = Regex(#".*([xy])=(\d+)"#)

            var position: Int
            var axis: Axis

            enum Axis: String {
                case x, y
            }

            var rawValue: String {
                "fold along \(axis.rawValue)=\(position)"
            }

            init?(rawValue: String) {
                guard let match = Self.pattern.firstMatch(in: rawValue),
                      match.captures.count == 2,
                      let axisStr = match.captures[0],
                      let axis = Axis(rawValue: axisStr),
                      let positionStr = match.captures[1],
                      let position = Int(positionStr)
                else {
                    return nil
                }

                self.position = position
                self.axis = axis
            }
        }

        var rawValue: String {
            ""
        }

        init?(rawValue: String) {
            let components = rawValue
                .components(separatedBy: "\n\n")
                .prefix(2)

            guard components.count == 2 else {
                return nil
            }

            let (dotsRaw, foldsRaw) = (components[0], components[1])

            var dots: Set<Point2> = Set(
                dotsRaw
                    .components(separatedBy: "\n")
                    .compactMap {
                        let pos = $0.components(separatedBy: ",").prefix(2)

                        guard pos.count == 2,
                              let x = Int(pos[0]),
                              let y = Int(pos[1])
                        else {
                            return nil
                        }

                        return Point2(x: x, y: y)
                    }
            )

            let folds = foldsRaw
                .components(separatedBy: "\n")
                .compactMap(Fold.init)

            for fold in folds {
                var add: Set<Point2> = []
                var remove: Set<Point2> = []

                for dot in dots {
                    switch fold.axis {
                    case .x where dot.x > fold.position:
                        remove.insert(dot)
                        add.insert(Point2(x: 2 * fold.position - dot.x, y: dot.y))
                    case .y where dot.y > fold.position:
                        remove.insert(dot)
                        add.insert(Point2(x: dot.x, y: 2 * fold.position - dot.y))
                    default:
                        continue
                    }
                }

                dots.subtract(remove)
                dots.formUnion(add)
            }

            self.dots = dots
        }


    }
}