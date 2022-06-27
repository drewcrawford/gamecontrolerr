use std::os::raw::c_long;

#[derive(Debug)]
pub struct GCKeyCode(pub c_long /* CFIndex */ );

extern "C" {
    static GCKeyCodeKeyA: c_long;

    static GCKeyCodeKeyB: c_long;
    static GCKeyCodeKeyC: c_long;
    static GCKeyCodeKeyD: c_long;
    static GCKeyCodeKeyE: c_long;
    static GCKeyCodeKeyF: c_long;
    static GCKeyCodeKeyG: c_long;
    static GCKeyCodeKeyH: c_long;
    static GCKeyCodeKeyI: c_long;
    static GCKeyCodeKeyJ: c_long;
    static GCKeyCodeKeyK: c_long;
    static GCKeyCodeKeyL: c_long;
    static GCKeyCodeKeyM: c_long;
    static GCKeyCodeKeyN: c_long;
    static GCKeyCodeKeyO: c_long;
    static GCKeyCodeKeyP: c_long;
    static GCKeyCodeKeyQ: c_long;
    static GCKeyCodeKeyR: c_long;
    static GCKeyCodeKeyS: c_long;
    static GCKeyCodeKeyT: c_long;
    static GCKeyCodeKeyU: c_long;
    static GCKeyCodeKeyV: c_long;
    static GCKeyCodeKeyW: c_long;
    static GCKeyCodeKeyX: c_long;
    static GCKeyCodeKeyY: c_long;
    static GCKeyCodeKeyZ: c_long;
    static GCKeyCodeKeyOne: c_long;
    static GCKeyCodeKeyTwo: c_long;
    static GCKeyCodeKeyThree: c_long;
    static GCKeyCodeKeyFour: c_long;
    static GCKeyCodeKeyFive: c_long;
    static GCKeyCodeKeySix: c_long;
    static GCKeyCodeKeySeven: c_long;
    static GCKeyCodeKeyEight: c_long;
    static GCKeyCodeKeyNine: c_long;
    static GCKeyCodeKeyZero: c_long;
    static GCKeyCodeReturnOrEnter: c_long;
    static GCKeyCodeEscape: c_long;
    static GCKeyCodeDeleteOrBackspace: c_long;
    static GCKeyCodeTab: c_long;
    static GCKeyCodeSpacebar: c_long;
    static GCKeyCodeHyphen: c_long;
    static GCKeyCodeEqualSign: c_long;
    static GCKeyCodeOpenBracket: c_long;
    static GCKeyCodeCloseBracket: c_long;
    static GCKeyCodeBackslash: c_long;
    static GCKeyCodeNonUSPound: c_long;
    static GCKeyCodeSemicolon: c_long;
    static GCKeyCodeQuote: c_long;
    static GCKeyCodeGraveAccentAndTilde: c_long;
    static GCKeyCodeComma: c_long;
    static GCKeyCodePeriod: c_long;
    static GCKeyCodeSlash: c_long;
    static GCKeyCodeCapsLock: c_long;
    static GCKeyCodeF1: c_long;
    static GCKeyCodeF2: c_long;
    static GCKeyCodeF3: c_long;
    static GCKeyCodeF4: c_long;
    static GCKeyCodeF5: c_long;
    static GCKeyCodeF6: c_long;
    static GCKeyCodeF7: c_long;
    static GCKeyCodeF8: c_long;
    static GCKeyCodeF9: c_long;
    static GCKeyCodeF10: c_long;
    static GCKeyCodeF11: c_long;
    static GCKeyCodeF12: c_long;
    static GCKeyCodeF13: c_long;
    static GCKeyCodeF14: c_long;
    static GCKeyCodeF15: c_long;
    static GCKeyCodeF16: c_long;
    static GCKeyCodeF17: c_long;
    static GCKeyCodeF18: c_long;
    static GCKeyCodeF19: c_long;
    static GCKeyCodeF20: c_long;
    static GCKeyCodePrintScreen: c_long;
    static GCKeyCodeScrollLock: c_long;
    static GCKeyCodePause: c_long;
    static GCKeyCodeInsert: c_long;
    static GCKeyCodeHome: c_long;
    static GCKeyCodePageUp: c_long;
    static GCKeyCodeDeleteForward: c_long;
    static GCKeyCodeEnd: c_long;
    static GCKeyCodePageDown: c_long;
    static GCKeyCodeRightArrow: c_long;
    static GCKeyCodeLeftArrow: c_long;
    static GCKeyCodeDownArrow: c_long;
    static GCKeyCodeUpArrow: c_long;
    static GCKeyCodeKeypadNumLock: c_long;
    static GCKeyCodeKeypadSlash: c_long;
    static GCKeyCodeKeypadAsterisk: c_long;
    static GCKeyCodeKeypadHyphen: c_long;
    static GCKeyCodeKeypadPlus: c_long;
    static GCKeyCodeKeypadEnter: c_long;
    static GCKeyCodeKeypad1: c_long;
    static GCKeyCodeKeypad2: c_long;
    static GCKeyCodeKeypad3: c_long;
    static GCKeyCodeKeypad4: c_long;
    static GCKeyCodeKeypad5: c_long;
    static GCKeyCodeKeypad6: c_long;
    static GCKeyCodeKeypad7: c_long;
    static GCKeyCodeKeypad8: c_long;
    static GCKeyCodeKeypad9: c_long;
    static GCKeyCodeKeypad0: c_long;
    static GCKeyCodeKeypadPeriod: c_long;
    static GCKeyCodeKeypadEqualSign: c_long;
    static GCKeyCodeNonUSBackslash: c_long;
    static GCKeyCodeApplication: c_long;
    static GCKeyCodePower: c_long;
    static GCKeyCodeInternational1: c_long;
    static GCKeyCodeInternational2: c_long;
    static GCKeyCodeInternational3: c_long;
    static GCKeyCodeInternational4: c_long;
    static GCKeyCodeInternational5: c_long;
    static GCKeyCodeInternational6: c_long;
    static GCKeyCodeInternational7: c_long;
    static GCKeyCodeInternational8: c_long;
    static GCKeyCodeInternational9: c_long;
    static GCKeyCodeLANG1: c_long;
    static GCKeyCodeLANG2: c_long;
    static GCKeyCodeLANG3: c_long;
    static GCKeyCodeLANG4: c_long;
    static GCKeyCodeLANG5: c_long;
    static GCKeyCodeLANG6: c_long;
    static GCKeyCodeLANG7: c_long;
    static GCKeyCodeLANG8: c_long;
    static GCKeyCodeLANG9: c_long;
    static GCKeyCodeLeftControl: c_long;
    static GCKeyCodeLeftShift: c_long;
    static GCKeyCodeLeftAlt: c_long;
    static GCKeyCodeLeftGUI: c_long;
    static GCKeyCodeRightControl: c_long;
    static GCKeyCodeRightShift: c_long;
    static GCKeyCodeRightAlt: c_long;
    static GCKeyCodeRightGUI: c_long;
}


impl GCKeyCode {
    pub fn a() -> Self { unsafe { Self(GCKeyCodeKeyA) }}
    pub fn b() -> Self { unsafe { Self(GCKeyCodeKeyB) }}
    pub fn c() -> Self { unsafe { Self(GCKeyCodeKeyC) }}
    pub fn d() -> Self { unsafe { Self(GCKeyCodeKeyD) }}
    pub fn e() -> Self { unsafe { Self(GCKeyCodeKeyE) }}
    pub fn f() -> Self { unsafe { Self(GCKeyCodeKeyF) }}
    pub fn g() -> Self { unsafe { Self(GCKeyCodeKeyG) }}
    pub fn h() -> Self { unsafe { Self(GCKeyCodeKeyH) }}
    pub fn i() -> Self { unsafe { Self(GCKeyCodeKeyI) }}
    pub fn j() -> Self { unsafe { Self(GCKeyCodeKeyJ) }}
    pub fn k() -> Self { unsafe { Self(GCKeyCodeKeyK) }}
    pub fn l() -> Self { unsafe { Self(GCKeyCodeKeyL) }}
    pub fn m() -> Self { unsafe { Self(GCKeyCodeKeyM) }}
    pub fn n() -> Self { unsafe { Self(GCKeyCodeKeyN) }}
    pub fn o() -> Self { unsafe { Self(GCKeyCodeKeyO) }}
    pub fn p() -> Self { unsafe { Self(GCKeyCodeKeyP) }}
    pub fn q() -> Self { unsafe { Self(GCKeyCodeKeyQ) }}
    pub fn r() -> Self { unsafe { Self(GCKeyCodeKeyR) }}
    pub fn s() -> Self { unsafe { Self(GCKeyCodeKeyS) }}
    pub fn t() -> Self { unsafe { Self(GCKeyCodeKeyT) }}
    pub fn u() -> Self { unsafe { Self(GCKeyCodeKeyU) }}
    pub fn v() -> Self { unsafe { Self(GCKeyCodeKeyV) }}
    pub fn w() -> Self { unsafe { Self(GCKeyCodeKeyW) }}
    pub fn x() -> Self { unsafe { Self(GCKeyCodeKeyX) }}
    pub fn y() -> Self { unsafe { Self(GCKeyCodeKeyY) }}
    pub fn z() -> Self { unsafe { Self(GCKeyCodeKeyZ) }}
    pub fn one() -> Self { unsafe { Self(GCKeyCodeKeyOne) }}
    pub fn two() -> Self { unsafe { Self(GCKeyCodeKeyTwo) }}
    pub fn three() -> Self { unsafe { Self(GCKeyCodeKeyThree) }}
    pub fn four() -> Self { unsafe { Self(GCKeyCodeKeyFour) }}
    pub fn five() -> Self { unsafe { Self(GCKeyCodeKeyFive) }}
    pub fn six() -> Self { unsafe { Self(GCKeyCodeKeySix) }}
    pub fn seven() -> Self { unsafe { Self(GCKeyCodeKeySeven) }}
    pub fn eight() -> Self { unsafe { Self(GCKeyCodeKeyEight) }}
    pub fn nine() -> Self { unsafe { Self(GCKeyCodeKeyNine) }}
    pub fn zero() -> Self { unsafe { Self(GCKeyCodeKeyZero) }}
    pub fn return_or_enter() -> Self { unsafe { Self(GCKeyCodeReturnOrEnter) }}
    pub fn escape() -> Self { unsafe { Self(GCKeyCodeEscape) }}
    pub fn delete_or_backspace() -> Self { unsafe { Self(GCKeyCodeDeleteOrBackspace) }}
    pub fn tab() -> Self { unsafe { Self(GCKeyCodeTab) }}
    pub fn spacebar() -> Self { unsafe { Self(GCKeyCodeSpacebar) }}
    pub fn hyphen() -> Self { unsafe { Self(GCKeyCodeHyphen) }}
    pub fn equal_sign() -> Self { unsafe { Self(GCKeyCodeEqualSign) }}
    pub fn open_bracket() -> Self { unsafe { Self(GCKeyCodeOpenBracket) }}
    pub fn close_bracket() -> Self { unsafe { Self(GCKeyCodeCloseBracket) }}
    pub fn backslash() -> Self { unsafe { Self(GCKeyCodeBackslash) }}
    pub fn non_us_pound() -> Self { unsafe { Self(GCKeyCodeNonUSPound) }}
    pub fn semicolon() -> Self { unsafe { Self(GCKeyCodeSemicolon) }}
    pub fn quote() -> Self { unsafe { Self(GCKeyCodeQuote) }}
    pub fn grave_accent_and_tilde() -> Self { unsafe { Self(GCKeyCodeGraveAccentAndTilde) }}
    pub fn comma() -> Self { unsafe { Self(GCKeyCodeComma) }}
    pub fn period() -> Self { unsafe { Self(GCKeyCodePeriod) }}
    pub fn slash() -> Self { unsafe { Self(GCKeyCodeSlash) }}
    pub fn capslock() -> Self { unsafe { Self(GCKeyCodeCapsLock) }}
    pub fn f1() -> Self { unsafe { Self(GCKeyCodeF1) }}
    pub fn f2() -> Self { unsafe { Self(GCKeyCodeF2) }}
    pub fn f3() -> Self { unsafe { Self(GCKeyCodeF3) }}
    pub fn f4() -> Self { unsafe { Self(GCKeyCodeF4) }}
    pub fn f5() -> Self { unsafe { Self(GCKeyCodeF5) }}
    pub fn f6() -> Self { unsafe { Self(GCKeyCodeF6) }}
    pub fn f7() -> Self { unsafe { Self(GCKeyCodeF7) }}
    pub fn f8() -> Self { unsafe { Self(GCKeyCodeF8) }}
    pub fn f9() -> Self { unsafe { Self(GCKeyCodeF9) }}
    pub fn f10() -> Self { unsafe { Self(GCKeyCodeF10) }}
    pub fn f11() -> Self { unsafe { Self(GCKeyCodeF11) }}
    pub fn f12() -> Self { unsafe { Self(GCKeyCodeF12) }}
    pub fn f13() -> Self { unsafe { Self(GCKeyCodeF13) }}
    pub fn f14() -> Self { unsafe { Self(GCKeyCodeF14) }}
    pub fn f15() -> Self { unsafe { Self(GCKeyCodeF15) }}
    pub fn f16() -> Self { unsafe { Self(GCKeyCodeF16) }}
    pub fn f17() -> Self { unsafe { Self(GCKeyCodeF17) }}
    pub fn f18() -> Self { unsafe { Self(GCKeyCodeF18) }}
    pub fn f19() -> Self { unsafe { Self(GCKeyCodeF19) }}
    pub fn f20() -> Self { unsafe { Self(GCKeyCodeF20) }}

    pub fn print_screen() -> Self { unsafe { Self(GCKeyCodePrintScreen) }}
    pub fn scroll_lock() -> Self { unsafe { Self(GCKeyCodeScrollLock) }}
    pub fn pause() -> Self { unsafe { Self(GCKeyCodePause) }}
    pub fn insert() -> Self { unsafe { Self(GCKeyCodeInsert) }}
    pub fn home() -> Self { unsafe { Self(GCKeyCodeHome) }}
    pub fn page_up() -> Self { unsafe { Self(GCKeyCodePageUp) }}
    pub fn delete_forward() -> Self { unsafe { Self(GCKeyCodeDeleteForward) }}
    pub fn end() -> Self { unsafe { Self(GCKeyCodeEnd) }}
    pub fn page_down() -> Self { unsafe { Self(GCKeyCodePageDown) }}
    pub fn right_arrow() -> Self { unsafe { Self(GCKeyCodeRightArrow) }}
    pub fn left_arrow() -> Self { unsafe { Self(GCKeyCodeLeftArrow) }}
    pub fn down_arrow() -> Self { unsafe { Self(GCKeyCodeDownArrow) }}
    pub fn up_arrow() -> Self { unsafe { Self(GCKeyCodeUpArrow) }}
    pub fn keypad_num_lock() -> Self { unsafe { Self(GCKeyCodeKeypadNumLock) }}
    pub fn keypad_slash() -> Self { unsafe { Self(GCKeyCodeKeypadSlash) }}
    pub fn keypad_asterisk() -> Self { unsafe { Self(GCKeyCodeKeypadAsterisk) }}
    pub fn keypad_hyphen() -> Self { unsafe { Self(GCKeyCodeKeypadHyphen) }}
    pub fn keypad_plus() -> Self { unsafe { Self(GCKeyCodeKeypadPlus) }}
    pub fn keypad_enter() -> Self { unsafe { Self(GCKeyCodeKeypadEnter) }}
    pub fn keypad_1() -> Self { unsafe { Self(GCKeyCodeKeypad1) }}
    pub fn keypad_2() -> Self { unsafe { Self(GCKeyCodeKeypad2) }}
    pub fn keypad_3() -> Self { unsafe { Self(GCKeyCodeKeypad3) }}
    pub fn keypad_4() -> Self { unsafe { Self(GCKeyCodeKeypad4) }}
    pub fn keypad_5() -> Self { unsafe { Self(GCKeyCodeKeypad5) }}
    pub fn keypad_6() -> Self { unsafe { Self(GCKeyCodeKeypad6) }}
    pub fn keypad_7() -> Self { unsafe { Self(GCKeyCodeKeypad7) }}
    pub fn keypad_8() -> Self { unsafe { Self(GCKeyCodeKeypad8) }}
    pub fn keypad_9() -> Self { unsafe { Self(GCKeyCodeKeypad9) }}
    pub fn keypad_0() -> Self { unsafe { Self(GCKeyCodeKeypad0) }}
    pub fn keypad_period() -> Self { unsafe { Self(GCKeyCodeKeypadPeriod) }}
    pub fn keypad_equal_sign() -> Self { unsafe { Self(GCKeyCodeKeypadEqualSign) }}
    pub fn keypad_non_us_backslash() -> Self { unsafe { Self(GCKeyCodeNonUSBackslash) }}
    pub fn application() -> Self { unsafe { Self(GCKeyCodeApplication) }}
    pub fn power() -> Self { unsafe { Self(GCKeyCodePower) }}
    pub fn international_1() -> Self { unsafe { Self(GCKeyCodeInternational1) }}
    pub fn international_2() -> Self { unsafe { Self(GCKeyCodeInternational2) }}
    pub fn international_3() -> Self { unsafe { Self(GCKeyCodeInternational3) }}
    pub fn international_4() -> Self { unsafe { Self(GCKeyCodeInternational4) }}
    pub fn international_5() -> Self { unsafe { Self(GCKeyCodeInternational5) }}
    pub fn international_6() -> Self { unsafe { Self(GCKeyCodeInternational6) }}
    pub fn international_7() -> Self { unsafe { Self(GCKeyCodeInternational7) }}
    pub fn international_8() -> Self { unsafe { Self(GCKeyCodeInternational8) }}
    pub fn international_9() -> Self { unsafe { Self(GCKeyCodeInternational9) }}
    pub fn lang_1() -> Self { unsafe { Self(GCKeyCodeLANG1) }}
    pub fn lang_2() -> Self { unsafe { Self(GCKeyCodeLANG2) }}
    pub fn lang_3() -> Self { unsafe { Self(GCKeyCodeLANG3) }}
    pub fn lang_4() -> Self { unsafe { Self(GCKeyCodeLANG4) }}
    pub fn lang_5() -> Self { unsafe { Self(GCKeyCodeLANG5) }}
    pub fn lang_6() -> Self { unsafe { Self(GCKeyCodeLANG6) }}
    pub fn lang_7() -> Self { unsafe { Self(GCKeyCodeLANG7) }}
    pub fn lang_8() -> Self { unsafe { Self(GCKeyCodeLANG8) }}
    pub fn lang_9() -> Self { unsafe { Self(GCKeyCodeLANG9) }}

    pub fn left_control() -> Self { unsafe { Self(GCKeyCodeLeftControl) }}
    pub fn left_shift() -> Self { unsafe { Self(GCKeyCodeLeftShift) }}
    pub fn left_alt() -> Self { unsafe { Self(GCKeyCodeLeftAlt) }}
    pub fn left_gui() -> Self { unsafe { Self(GCKeyCodeLeftGUI) }}

    pub fn right_control() -> Self { unsafe { Self(GCKeyCodeRightControl) }}
    pub fn right_shift() -> Self { unsafe { Self(GCKeyCodeRightShift) }}
    pub fn right_alt() -> Self { unsafe { Self(GCKeyCodeRightAlt) }}
    pub fn right_gui() -> Self { unsafe { Self(GCKeyCodeRightGUI) }}
}
