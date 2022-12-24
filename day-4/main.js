import { readFileSync } from 'fs';

const input = readFileSync(process.argv[2], { encoding: 'utf-8' });

const pairs = input.split('\n');

class Range {
	start;
	end;

	constructor(pattern) {
		let [start, end] = pattern.split('-');

		this.start = parseInt(start);
		this.end = parseInt(end);
	}

	overlaps(other) {
		let first = this.toArray();
		let second = other.toArray();
		return (
			Array.from(new Set([...first, ...second])).length !=
			first.length + second.length
		);
	}

	toArray() {
		let array = [];
		for (let i = this.start; i <= this.end; i++) {
			array.push(i);
		}
		return array;
	}

	isContainedBy(other) {
		return other.start <= this.start && other.end >= this.end;
	}

	contains(other) {
		return this.start <= other.start && this.end >= other.end;
	}
}

const result = pairs
	.map((pair) => {
		let [firstElf, secondElf] = pair.trim().split(',');
		let [range, secondRange] = [new Range(firstElf), new Range(secondElf)];

		/* Part 1
        let contained = range.contains(range2) || range.isContainedBy(range2); 
        */
		/* Part 2 */
		return range.overlaps(secondRange);
	})
    // Filters out each pair that doesn't overlap
	.filter((pair) => pair);

console.log(result.length);
