
// 14 miliseconed 
// function majorityElement(nums: number[]) {
//    const m = nums.length / 2;

//    const map = new Map<number, number>();
   
//     // Add all the variable to the map
//    for (const n of nums) {
//         if(!map.has(n)) {
//             map.set(n, 0)
//         }
//         map.set(n, map.get(n)! + 1)
//    }

//    let highest = 0;
//    for (const [key, val] of map.entries()) {
//     if(val > m) {

//         if (val > highest) {
//             highest = val;
//             return key;
//         }
//     }
//    }
// };

// 10ms
// function majorityElement(nums: number[]) {
//     const m = nums.length / 2;
 
//     const countArray: [number, number][] = [];

//     // Add all the variables to the countArray
//     for (const n of nums) {
//         const index = countArray.findIndex(([num]) => num === n);
//         if (index === -1) {
//             countArray.push([n, 1]);
//         } else {
//             countArray[index][1]++;
//         }
//     }
 
//     let highest = 0;
//     for (const [key, val] of countArray) {
//      if(val > m) {
 
//          if (val > highest) {
//              highest = val;
//              return key;
//          }
//      }
//     }
//  };

function majorityElement(nums: number[]) {
    let candidate;
    let count = 0;
    
    for (const num of nums) {
        if (count === 0) {
            console.log("num", count)
            candidate = num;
        }
        count += (num === candidate) ? 1 : -1
        console.log("num & candidate",{ candidate, num, count})
    }
    
    return candidate;
};

