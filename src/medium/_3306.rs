pub struct Solution;

impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        let mut vowel_tbl: [u16; 128] = [0; 128];
        let mut const_tbl: [u16; 128] = [0; 128];

        for &vowel in "aeiou".as_bytes() {
            vowel_tbl[vowel as usize] = 1;
        }

        let mut res: i64 = 0;
        let mut current_k: u32 = 0;
        let mut vowel_count: u8 = 0;
        let mut extra_left: i64 = 0;

        let n: usize = word.len();
        let k: u32 = k as u32;
        let mut lp: usize = 0;
        let mut rp: usize = 0;

        let word: &[u8] = word.as_bytes();

        while rp < n {
            let rchar: usize = word[rp] as usize;

            if vowel_tbl[rchar] == 1 {
                if const_tbl[rchar] == 0 {
                    vowel_count += 1;
                }

                const_tbl[rchar] += 1;
            } else {
                current_k += 1;
            }

            while current_k > k {
                let lchar: usize = word[lp] as usize;

                if vowel_tbl[lchar] == 1 {
                    const_tbl[lchar] -= 1;

                    if const_tbl[lchar] == 0 {
                        vowel_count -= 1;
                    }
                } else {
                    current_k -= 1;
                }

                lp += 1;
                extra_left = 0;
            }

            while vowel_count == 5
                && current_k == k
                && lp < rp
                && vowel_tbl[word[lp] as usize] == 1
                && const_tbl[word[lp] as usize] > 1
            {
                extra_left += 1;
                const_tbl[word[lp] as usize] -= 1;
                lp += 1;
            }

            if current_k == k && vowel_count == 5 {
                res += 1 + extra_left;
            }

            rp += 1;
        }

        res
    }

    pub fn count_of_substrings_alt(word: String, k: i32) -> i64 {
        let mut frequencies: [[i32; 128]; 2] = [[0; 128], [0; 128]];

        for &vowel in "aeiou".as_bytes() {
            frequencies[0][vowel as usize] = 1;
        }

        let mut response: i64 = 0;
        let mut current_k: usize = 0;
        let mut vowels: i32 = 0;
        let mut extra_left: i64 = 0;
        let mut lp: usize = 0;
        let k: usize = k as usize;

        let word_bytes: &[u8] = word.as_bytes();

        for rp in 0..word_bytes.len() {
            let right_char: usize = word_bytes[rp] as usize;

            if frequencies[0][right_char] == 1 {
                if frequencies[1][right_char] == 0 {
                    vowels += 1;
                }

                frequencies[1][right_char] += 1;
            } else {
                current_k += 1;
            }

            while current_k > k {
                let left_char: usize = word_bytes[lp] as usize;

                if frequencies[0][left_char] == 1 {
                    frequencies[1][left_char] -= 1;

                    if frequencies[1][left_char] == 0 {
                        vowels -= 1;
                    }
                } else {
                    current_k -= 1;
                }

                lp += 1;
                extra_left = 0;
            }

            while vowels == 5
                && current_k == k
                && lp < rp
                && frequencies[0][word_bytes[lp] as usize] == 1
                && frequencies[1][word_bytes[lp] as usize] > 1
            {
                extra_left += 1;
                frequencies[1][word_bytes[lp] as usize] -= 1;
                lp += 1;
            }

            if current_k == k && vowels == 5 {
                response += 1 + extra_left;
            }
        }

        response
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
