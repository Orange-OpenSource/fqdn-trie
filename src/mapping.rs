// each letter is associated to a code with two digits (primary and secondary)
// in the trie, such a letter leads to a primary branching (with the first digit on the code) and,
// if needed, a second branching (with the last digit)
//
// the association of the codes will done according to an estimation of the frequency of letters in FQDN
// in order to minimize the needs of the secondary branching

// comparison with the flat association (previous version):
// - memory usage is equivalent if no more than 3 secondary branching exist for primary ones (in average)
// - lookup performance is exactly the same if there is no secondary branching
// some tests should be scheduled...



// value of the _ depends if we apply the rfc strictly or not
#[cfg(feature="domain-name-without-special-chars")] const UNDER: (u8,u8) = (0,0);
#[cfg(not(feature="domain-name-without-special-chars"))] const UNDER: (u8,u8) = (1,2);

// if more special chars are needed, the remaining available codes are: (1,3), (1,4), (1,5), (1,6)
// (this should be have a negligible impact on the memory use and on performances)

// if it is not enough, then the code (0,1) to (0,6) could be used
// (in this case, the impact could be greater since branching on (0,*) becomes possible)

const MAPPING: [(u8,u8);256] = [
    (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0),     (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0),
    (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0),     (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0),
    (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0),     (0,0), (0,0), (0,0), (0,0), (0,0), (3,3), (0,0), (0,0), // '-' (dash)
    (3,5), (3,4), (3,6), (2,1), (2,3), (2,4), (2,5), (2,2),     (1,1), (2,6), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), // 0-9
    (0,0), (6,3), (5,4), (2,0), (6,5), (6,2), (4,1), (5,3),     (5,6), (6,1), (4,6), (4,0), (5,5), (5,0), (6,0), (1,0), // A-O
    (5,2), (3,1), (6,6), (5,1), (6,4), (3,0), (4,2), (4,4),     (4,5), (4,3), (3,2), (0,0), (0,0), (0,0), (0,0), UNDER, // P-Z and '_' (underscore)
    (0,0), (6,3), (5,4), (2,0), (6,5), (6,2), (4,1), (5,3),     (5,6), (6,1), (4,6), (4,0), (5,5), (5,0), (6,0), (1,0), // a-o
    (5,2), (3,1), (6,6), (5,1), (6,4), (3,0), (4,2), (4,4),     (4,5), (4,3), (3,2), (0,0), (0,0), (0,0), (0,0), (0,0), // p-z
    (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0),     (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0),
    (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0),     (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0),
    (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0),     (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0),
    (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0),     (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0),
    (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0),     (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0),
    (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0),     (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0),
    (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0),     (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0),
    (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0),     (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0),
];

#[inline]
pub(crate) fn byte2code(byte: u8) -> (u8, u8)
{
    MAPPING[byte as usize]
}

#[inline]
pub(crate) fn code2byte(code: (u8,u8)) -> Option<u8>
{
    MAPPING.iter()
        .enumerate()
        .find(|(_,&c)| c == code)
        .map(|(i, _)| i as u8)
}

/* last fqdn char: m u t z g o k
.,  ,  ,  ,  ,  ,  ,
o, 8, _,  ,  ,  ,  ,
c, 3, 7, 4, 5, 6, 9,
u, q, z, -, 1, 0, 2,
k, f, v, y, w, x, j,
m, s, p, g, b, l, h,
n, i, e, a, t, d, r,
 */
// a c d e g h i j k l m n o r s t u v w x y z