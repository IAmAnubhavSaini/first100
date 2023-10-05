function medianForEvenElementList(list) {
    const len = list.length;
    const mid = Math.floor(len/2);
    return (list[mid-1] + list[mid]) / 2;
}

function medianForOddElementList(list) {
    const len = list.length;
    const mid = Math.floor(len/2);
    return list[mid];
}

function median(list) {
    const len = list.length;
    const sortedList= list.sort((a, b) => a - b);
    if(len % 2 === 0) {
        return medianForEvenElementList(sortedList);
    } else {
        return medianForOddElementList(sortedList);
    }
}

function run() {
    let list = [2, 3, 4, 2, 1, 3, 5, 5, 23, 2, 2, 5, 2, 3];
    let med = median(list);
    console.log({med});

    list = [3, 2, 1];
    med = median(list);
    console.log({med});

    list = [4, 3, 2, 1];
    med = median(list);
    console.log({med});

    list = [5, 4, 3, 2, 1];
    med = median(list);
    console.log({med});

    list = [1, 2, 8, 6, 4, 2, 1, 2, 54, 43, 46];
    med = median(list);
    console.log({med});
}

run();
