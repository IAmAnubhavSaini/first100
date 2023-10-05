function unique(list) {
    const newList = [];
    for(let i = 0; i < list.length ; i++) {
        const num = list[i];
        if(!newList.includes(num)) {
            newList.push(num);
        }
    }
    return newList;
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
