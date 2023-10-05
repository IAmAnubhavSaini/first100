function compare(a, b) {
    const _a = a.toLowerCase();
    const _b = b.toLowerCase();
    console.log({_a, _b, alb: _a < _b});
    return _a < _b ? -1 : _a === _b ? 0 : 1;
    // return _a - _b; // this works with numbers okay; but not with strings
}
function sortCaseInsensitive(list) {
    list.sort(compare);
    return list;
}

function main() {
    const usernames = ["ToMato", "tomato",  "amy", "potato", "Potato", "peter", "pyter"];
    const cloned = [...usernames];
    console.log({usernames, sorted: sortCaseInsensitive(cloned)});
    // console.log({usernames, cloned});
}

main();
