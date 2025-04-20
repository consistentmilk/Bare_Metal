/*
Intuition:
1. We must count all substrings containing exactly k consonants and at least one of each vowel (a, e, i, o, u).
2. A sliding window can track the number of consonants (current_k), the distinct vowels seen (vowel_count), and their frequencies (const_tbl).
3. Whenever consonants exceed k, we advance the left pointer (lp) until the window is valid again, resetting any extra leading-vowel count.
4. Once exactly k consonants and all 5 vowels are in the window, additional leading vowels (extra_left) yield extra valid substrings ending at the current right pointer.

Algorithm:
1. Initialize tables: `vowel_tbl` to mark vowels, `const_tbl` to count their occurrences in the window.
2. Slide `rp` from 0 to n−1:
   a. Update vowel frequency or consonant count for the character at `rp`.
   b. While consonant count > k, advance `lp`, decrementing counts and resetting `extra_left`.
   c. If window has all 5 vowels and exactly k consonants, shift `lp` over extra leading vowels to compute `extra_left`.
   d. Add `1 + extra_left` to the result when valid.
3. Return the cumulative result.

Time Complexity:
- O(n), where n = word.len(), since each character is visited at most twice (once by rp, once by lp).

Space Complexity:
- O(1), using fixed-size tables of length 128 and constant extra variables.
*/

pub struct Solution;

impl Solution {
    // Count substrings with exactly k consonants and all 5 vowels.
    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        // Table marking vowels ('a','e','i','o','u') as 1.
        let mut vowel_tbl: [u16; 128] = [0; 128];

        // Table counting frequencies of vowels in the current window.
        let mut const_tbl: [u16; 128] = [0; 128];

        // Mark each vowel in vowel_tbl.
        for &vowel in "aeiou".as_bytes() {
            const_tbl[vowel as usize]; // ensure vowel index is valid
            vowel_tbl[vowel as usize] = 1;
        }

        // Result accumulator for valid substrings.
        let mut res: i64 = 0;

        // Current count of consonants in the window.
        let mut current_k: u32 = 0;

        // Count of distinct vowels seen in the window.
        let mut vowel_count: u8 = 0;

        // Extra leading vowels beyond the first that still yield valid substrings.
        let mut extra_left: i64 = 0;

        // Length of the input string.
        let n: usize = word.len();

        // Target consonant count converted to u32.
        let k: u32 = k as u32;

        // Left and right pointers for the sliding window.
        let mut lp: usize = 0;
        let mut rp: usize = 0;

        // Working byte slice of the input word.
        let word_bytes: &[u8] = word.as_bytes();

        // Expand the window with rp until the end of the word.
        while rp < n {
            // Map current character to its ASCII index.
            let rchar: usize = word_bytes[rp] as usize;

            // If it's a vowel, update its count and distinct-vowel tally.
            if vowel_tbl[rchar] == 1 {
                if const_tbl[rchar] == 0 {
                    vowel_count += 1;
                }
                const_tbl[rchar] += 1;
            } else {
                // Otherwise, it's a consonant: increment consonant count.
                current_k += 1;
            }

            // Shrink window from the left while consonants exceed k.
            while current_k > k {
                // Character being removed at lp.
                let lchar: usize = word_bytes[lp] as usize;

                if vowel_tbl[lchar] == 1 {
                    // Decrement vowel frequency, adjust distinct count if dropped to zero.
                    const_tbl[lchar] -= 1;
                    if const_tbl[lchar] == 0 {
                        vowel_count -= 1;
                    }
                } else {
                    // Removing a consonant decreases current_k.
                    current_k -= 1;
                }

                // Advance left pointer and reset extra leading vowels.
                lp += 1;
                extra_left = 0;
            }

            // If window is valid, shift lp over extra leading vowels to count extra substrings.
            while vowel_count == 5
                && current_k == k
                && lp < rp
                && vowel_tbl[word_bytes[lp] as usize] == 1
                && const_tbl[word_bytes[lp] as usize] > 1
            {
                // Each extra vowel at lp yields one additional valid substring.
                extra_left += 1;

                // Decrement its frequency and advance lp.
                const_tbl[word_bytes[lp] as usize] -= 1;
                lp += 1;
            }

            // When exactly k consonants and all vowels are present, add to result.
            if current_k == k && vowel_count == 5 {
                res += 1 + extra_left;
            }

            // Advance right pointer to consider the next character.
            rp += 1;
        }

        // Return total count of valid substrings.
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3306_1() {
        let word: String = "aeioqq".into();
        let k: i32 = 1;
        let expected: i64 = 0;
        let res: i64 = Solution::count_of_substrings(word, k);

        assert_eq!(expected, res);
    }

    #[test]
    fn test_3306_2() {
        let word: String = "aeiou".into();
        let k: i32 = 0;
        let expected: i64 = 1;
        let res: i64 = Solution::count_of_substrings(word, k);

        assert_eq!(expected, res);
    }

    #[test]
    fn test_3306_3() {
        let word: String = "ieaouqqieaouqq".into();
        let k: i32 = 1;
        let expected: i64 = 3;
        let res: i64 = Solution::count_of_substrings(word, k);

        assert_eq!(expected, res);
    }

    #[test]
    fn test_3306_4() {
        let word: String = "bcbcciiccibcbooouoiciuuoueeicoouucdcddeduiuiubboodudeacdieeueocaiadboudduoiociueeicidcuububaeeiuacaaubccauaicoobediaceboacoeebdciudadiuaiaieeeuabiicoecaaidaeidcuuaubbeiceecuuodcaeibdacoiiibidebdauaiuciddcuaueodeddicduoccciibucddoaedcoaucoiiuooeidaeibcedciadbiucibaoebuueiciiabbauaaidacioeadeaeeuiudcdaedduoeceaacebeieiuocuoaeiaocccudbbaeoccobcciecbccabobibouoaiccabcicdducoiadeuuodoceoeccicauedbbiebuiiecaeiaaaociddcudeueuuauceacoedicaeccobcoedoeoddecoiudidaobdboauebdecdoudidiooieaiedcuiocuiiudaaeeecuuiibaobceioiucoeaobbocdbocodeeouddedeeaiuoibaacuieeeiaoeaoauobbaooucuaduoaciacddaieobebabobceidiiodeobdbuciccaieeooudbabebuuabaobduebudadidauoibabacduiiaicdodbeoiabdiceciioooduuiddaoiiccbaoeoeoaaaidcicdoooeiciodiebaaudeuocabaaebaiiaeeaaiiadbciecuiabbducobduiueiuidaooaaodeddacbideeeddbbooidobdbdeacauooedcboeabecibiabucoccdooecoaeuoocebdooeuccdceaouoiabaieeodubdudeooeuucicbcoubuieideoiuaobobecdaduuidbcioidaaocdcbouobudabcacobceeoiaboouddcodabboueiodudadocdbuccodaoobbeoccocdcocoiibodeoubibaudcdadoaobouaieueouioeuecbuboboebuiaoieiooooubdobiuoeouucduuaeiebuuibciiiccaddduaauuobeiuodauiaaocuaabddaabcadoiaooibaiaocoeuuaieiueiaeauaeooeoiiduceauuobbobceobieobodbedaooaouucdbceeucbbiaudbadduaidbcbadcucudaocbdbuceeuadibiibccoucuueeuudbuubbaicecbabiaiiaaucdubbbboedceubccoocauioeiceieccoiiaecdaiccociuiocuuebucuoeeiubibaidudbibecdaaiddduiuceuueebeodbuoudacbceeciabcocdecbbebbacbeeiocioiedccooocieabobdiecuobuoueicuidebbaboiuuuocabeieoocbcidodocdaaeaaiaicaodbudioeadcieacuccddociiddeobibuddibibbbuccccacaoddbeoeduobuoboiecoocbeodicbibecoccdcidaoabeuuueiduibododiduauaueoaiudbdcodbubcacbioobacboiedodbbdcaecbacebcobccooauaudcuiiuouedicuccuceeccedddbiecucecebuuuuboaiaiaooebbeuoeiddicocaooauauuiuueuioiauaabuauaebdcaaaoobiouiiieouoiubibcdiiociocaecadaiaauoaibeeaacueddeubooaibaeaobadeeaaabaocucboocbaiaucicidaebuddicodeediouoiecaiiuoiuaebbadabeoboiciuieiiucccccobbiceaaobeeeebduuiaocddeuaoaobuicuioaobiciaiboeoabcuubdioidbaiboabbuaaabecadiiebeaebbdoacacicdacuaauobcodeaouuaaiiceadbieuiceabccbboeeeoibecueiaiioubdecbbdobdooiaebecucibcudbbodbeudibobdaiaoieieddbbaaeduoiocbbobdducbdeiadducaeuieoouebdauuaudubdiaeiubaaaioeoioocbccbccdcceaduibeaoebaeecdodbueiudecacauboecddacbdbdebdaoooidoiiiciucaoadbauoudbabbuaiiaaoouieuieebadoiaiiudcuiucbiecioudicioudcaoauiecboeiaaaaiaaiaoubcoidaeueduduaubbocoaiuudbubaodcooaoduacbooaiouacadebcuoibeuoouibbabaeuacidacoiuceieeuceeuuacuebuiiiaiucbbebcbecbicduibeoaieabiaeebddciboabbecedauaabceoiiiaabbiuobbeaabibdcaoaodueaoicoadubcbuboibebubiaacoiedoodooaabdcbiududibeedioiuuauaiieebuobiocdidedbicaaciuceeddocdidaoaeiiueuacadeaeodoceoaeoaoucdaacouiebeiaaoecbiicdaodidaacdcdbbaidbauiiuoaiidiceucciuobcbdabecuaubiddebuiaiccbocicbabodccoeiieudaeueieuacoccbcoidcdaouuoioiudbuuuioddooceucbucoaodcuceueeboeciooibbdoodcdaueaboiaedooaidccauioocuibubibucbuiccdaocdcioiecuibbudedooauiiaddibcbcbcccddaeuudbddioobbcoeaibiceuiuoebcbuaooeeebuoiibdbicubiioucdububibeccuacdboddcedcbdabbeebdaeoioabcubaeaiabaodiibuaebaooodobiuiocaabdacdoeuoicucieuboacuceideeuoadebuoadbiaducooueoaaiiiouicioidieoaucadacbdddioaiacueiaabuubciuddcacodoocboaubbddaboucoeoodueauabeiaiedauddiiceccoeuueodccuauiccodocoaauuceduedcoidouuiaicbiduaauouodoodueaueeaidaoueecicddbobdbaouiiecaebodubceeacdebeddobccabeobucddcauoaocueubicebbebeccucuiuacabbdcouiedaooudeiooiudduedccciedbeiducbdicabieedcbcedauuudbbuuddcobaaaeicaebeiobdeccduiibecadooudiobcaidodaobicebddiuaduuooudcaoaceoiiaoudcccciaaaauubuooaeubuuooeuadedobucuueodococucieoieuabboicducuoeoauecaooiueauaoceocueedoacibaoiduiebicuceooaoedbbceuaoiuaiccbaccuoccaecaiuooioeebodeebcdoueddoiideeocudceciobiidbebcubcbbbadoiudbiccduoebcbobcecoadddboioiiiiiiauciababcidodbicboeeiuouoibieocdeduiieceocboaucibooidoibbciedduuiieucaiducuoboucddaocduoauiiiudioobduicecoaadbcaaecoieodiaaeieubcauebbabaduceueeaeoeddueiccdudebdciiocuobbdcdabebddidoaduauudecbbauaieocucobeuaaiedbdcaoebocbedddiabiibduuueoioaabocedebdaadceeeiabaooadcdbucaaoabibieueebiudcdoeaduaoacuucaucbeuididibaceobbadoaeiuciudcudeiouieeueobouucueaducuabiibiuuadoccabdbcdaboeucecbaciiacioioaaeaucidcubabceauubdidcdcduuoceuboabcuoeaabbuoioeuccauiuudcbdobedbibiubccdeceeuoiuccabiocibdoocdboouabbubdeoecioadaeoboddbbbubaeaobcucauaidaeccaedcoooeeddboaidbbideeaooudeubeieiieddaoodcaubdiciiccuidudidiediuaidabeiiedbbeibdbucebcddcouoiodocbbaueddouicieebbdbddbeobuediadoaduoodoebeaecbcoueedaieccuubaaboaoieddidcieccbdiacdiicbuobdocbacueiobdcoducuiudebdedduoieuudeaudddaeiooueaeauaoaodibauddaaudbuadaodaubdidacbcoouoibdocobuodauuoucoobbiaueicdobidbabuuiduiouaabcbdebbcedbicicbeauuaucodicadeobuiecceoiccaacobcdiaoicabudbbbuacccabbaueeobdoucabaodbcbbaicuabcidicedcaedodcbubbicbaaadbbdiauaueciubbddbueedibbubiudbaciuuiibdaiaiocdicoaauabuubudboouiudeioaaadcaeebddebodouiioaecucdcaaibueboiauiiedoabdoddieodbceididbauocabocebdaieaduucdoiddceouidbucbauidaeaiodeboboacbieoubicecucaeieuibcouoeeaaiciuadadcbccedccuodeeodcooibeicuiubdbaeiedeabbdubebdbeiecdaiababoacuabaoodbauaacobioidecueueoaeceoecoodeuiedeiiebedbbaibiabucbdbuoauucdauobocaocecbeueaaccoacuouiieioaeibeeacubceddeuubbudiaudibddeaauiudidicaoiuebuuudbidoucacbebcaiuioobe".into();
        let k: i32 = 890;
        let expected: i64 = 6902;
        let res: i64 = Solution::count_of_substrings(word, k);

        assert_eq!(expected, res);
    }
}
