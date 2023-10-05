function unique(list) {
    return Array.from(new Set(list));
}

function main() {
    let list = [1, 2, 3, 4, 5, 6];
    console.log({list, unique: unique(list)});
    list = [1, 2, 3, 4, 5, 6, 1, 2, 3, 4, 5, 6];
    console.log({list, unique: unique(list)});
    list = [1, 6, 5, 4, 3, 2, 1, 2, 3, 4, 5, 6];
    console.log({list, unique: unique(list)});
}
main();
