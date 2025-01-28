function maxScore(s: string): number {
    let maxScore = 0;
    for (let i = 0; i < s.length; i++) {
        const left = s.slice(0, i + 1).split('').filter(char => char === '0');
        const right = s.slice(i, s.length).split('').filter(char => char === '1');

        console.log( right, left, i, s.length);

        if(left.length  || right.length ) {
            
        const currentScore = left.length + right.length;
    
        if (currentScore > maxScore) {
            maxScore = currentScore;
        }       
        }   
    }
    return maxScore
};

console.log(maxScore("1111"))