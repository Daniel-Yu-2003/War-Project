#[cfg(test)]
mod tests {
    // we adjust the path to:
    use super::super::{*};

    const T1: [u8; 52] = [1,1,1,1,13,13,13,13,11,11,11,11,12,12,12,12,10,10,10,10,9,9,9,9,7,7,7,7,8,8,8,8,6,6,6,6,5,5,5,5,4,4,4,4,3,3,3,3,2,2,2,2]; 
    const R1: [u8; 52] = [1,1,1,1,13,13,13,13,12,12,12,12,11,11,11,11,10,10,10,10,9,9,9,9,8,8,8,8,7,7,7,7,6,6,6,6,5,5,5,5,4,4,4,4,3,3,3,3,2,2,2,2];
    const T2: [u8; 52] = [1,13,1,13,1,13,1,13,12,11,12,11,12,11,12,11,10,9,10,9,10,9,10,9,8,7,8,7,8,7,8,7,6,5,6,5,6,5,6,5,4,3,4,3,4,3,4,3,2,2,2,2];
    const R2: [u8; 52] = [4,3,2,2,2,2,4,3,4,3,4,3,6,5,6,5,6,5,6,5,8,7,8,7,8,7,8,7,10,9,10,9,10,9,10,9,12,11,12,11,12,11,12,11,1,13,1,13,1,13,1,13];
    const T3: [u8; 52] = [13,1,13,1,13,1,13,1,11,12,11,12,11,12,11,12,9,10,9,10,9,10,9,10,7,8,7,8,7,8,7,8,5,6,5,6,5,6,5,6,3,4,3,4,3,4,3,4,2,2,2,2];
    const R3: [u8; 52] = [4,3,2,2,2,2,4,3,4,3,4,3,6,5,6,5,6,5,6,5,8,7,8,7,8,7,8,7,10,9,10,9,10,9,10,9,12,11,12,11,12,11,12,11,1,13,1,13,1,13,1,13];
    const T4: [u8; 52] = [10,11,12,13,1,2,3,4,5,6,7,8,9,10,11,12,13,1,2,3,4,5,6,7,8,9,10,11,12,13,1,2,3,4,5,6,7,8,9,10,11,12,13,1,2,3,4,5,6,7,8,9];
    const R4: [u8; 52] = [1,1,13,12,9,5,11,4,9,3,8,7,7,2,13,10,12,5,10,4,9,6,8,3,1,1,13,12,7,5,11,4,9,3,8,6,7,2,13,10,12,5,11,11,10,8,6,4,6,3,2,2];
    const T5: [u8; 52] = [1,2,3,4,5,6,7,8,9,10,11,12,13,1,2,3,4,5,6,7,8,9,10,11,12,13,1,2,3,4,5,6,7,8,9,10,11,12,13,1,2,3,4,5,6,7,8,9,10,11,12,13];
    const R5: [u8; 52] = [1,10,13,8,11,9,8,7,11,8,13,7,13,6,12,6,9,5,8,5,7,4,7,4,11,6,12,10,6,3,2,2,12,5,9,3,10,4,9,2,10,3,5,2,1,1,1,13,12,11,4,3];

    const T6: [u8; 52] = [1,2,1,2,3,4,3,4,5,6,5,6,7,8,7,8,9,10,9,10,11,12,11,12,13,13,1,2,1,2,3,4,3,4,5,6,5,6,7,8,7,8,9,10,9,10,11,12,11,12,13,13];
    const R6: [u8; 52] = [1,12,12,10,11,8,11,4,11,4,11,4,10,4,9,3,1,8,13,8,9,3,7,3,10,5,9,2,8,5,6,2,13,3,7,2,12,7,7,6,6,2,1,12,1,9,13,5,13,6,10,5];

    const T7: [u8; 52] = [1,2,3,4,5,6,1,2,3,4,5,6,7,8,9,10,11,12,7,8,9,10,11,12,13,13,1,2,3,4,5,6,1,2,3,4,5,6,7,8,9,10,11,12,7,8,9,10,11,12,13,13];
    const R7: [u8; 52] = [1,12,9,8,11,11,10,8,8,7,8,7,10,4,7,2,1,11,10,6,6,4,13,12,5,4,13,12,11,9,1,12,13,9,10,9,7,6,1,5,6,3,5,3,13,3,5,2,4,3,2,2];
	
    const T8: [u8; 52] =  [13,12,11,10,9,8,7,6,5,4,3,2,1,13,12,11,10,9,8,7,6,5,4,3,2,1,13,12,11,10,9,8,7,6,5,4,3,2,1,13,12,11,10,9,8,7,6,5,4,3,2,1];
    const R8: [u8; 52] =  [1,9,13,7,13,6,12,11,11,8,4,3,11,6,9,2,9,8,8,5,9,5,8,2,1,10,12,7,13,10,7,5,11,6,6,4,1,12,5,3,13,10,4,3,1,10,4,2,12,7,3,2];
	
    const T9: [u8; 52] =  [1,3,2,4,5,7,6,8,9,11,10,12,13,13,1,3,2,4,5,7,6,8,9,11,10,12,1,3,2,4,5,7,6,8,9,11,10,12,1,3,2,4,5,7,6,8,9,11,10,12,13,13];
    const R9: [u8; 52] =  [12,11,11,8,13,5,9,5,9,5,8,3,1,3,10,3,11,5,7,4,1,10,8,7,13,9,12,4,10,7,7,2,1,10,12,9,8,6,4,2,13,4,6,3,1,6,12,2,13,6,11,2];
	
    const T10: [u8; 52] = [1,1,1,1,3,3,3,3,2,2,2,2,4,4,4,4,5,5,5,5,7,7,7,7,6,6,6,6,8,8,8,8,9,9,9,9,11,11,11,11,10,10,10,10,12,12,12,12,13,13,13,13];
    const R10: [u8; 52] = [1,1,1,1,13,13,13,13,12,12,12,12,11,11,11,11,10,10,10,10,9,9,9,9,8,8,8,8,7,7,7,7,6,6,6,6,5,5,5,5,4,4,4,4,3,3,3,3,2,2,2,2];
	

    #[test] fn shuf1() { assert_eq!(deal(&T1), R1); }
    #[test] fn shuf2() { assert_eq!(deal(&T2), R2); }
    #[test] fn shuf3() { assert_eq!(deal(&T3), R3); }
    #[test] fn shuf4() { assert_eq!(deal(&T4), R4); }
    #[test] fn shuf5() { assert_eq!(deal(&T5), R5); }
    #[test] fn shuf6() { assert_eq!(deal(&T6), R6); }
    #[test] fn shuf7() { assert_eq!(deal(&T7), R7); }
    #[test] fn shuf8() { assert_eq!(deal(&T8), R8); }
    #[test] fn shuf9() { assert_eq!(deal(&T9), R9); }
    #[test] fn shuf10() { assert_eq!(deal(&T10), R10); }

    
}

