// File generated automatically by build.rs, do not modify.
use crate::interrupts::{set_idt_gate, IDTFlagNumeric};

extern "cdecl" {
    pub fn isr0 ();
    pub fn isr1 ();
    pub fn isr2 ();
    pub fn isr3 ();
    pub fn isr4 ();
    pub fn isr5 ();
    pub fn isr6 ();
    pub fn isr7 ();
    pub fn isr8 ();
    pub fn isr9 ();
    pub fn isr10 ();
    pub fn isr11 ();
    pub fn isr12 ();
    pub fn isr13 ();
    pub fn isr14 ();
    pub fn isr15 ();
    pub fn isr16 ();
    pub fn isr17 ();
    pub fn isr18 ();
    pub fn isr19 ();
    pub fn isr20 ();
    pub fn isr21 ();
    pub fn isr22 ();
    pub fn isr23 ();
    pub fn isr24 ();
    pub fn isr25 ();
    pub fn isr26 ();
    pub fn isr27 ();
    pub fn isr28 ();
    pub fn isr29 ();
    pub fn isr30 ();
    pub fn isr31 ();
    pub fn isr32 ();
    pub fn isr33 ();
    pub fn isr34 ();
    pub fn isr35 ();
    pub fn isr36 ();
    pub fn isr37 ();
    pub fn isr38 ();
    pub fn isr39 ();
    pub fn isr40 ();
    pub fn isr41 ();
    pub fn isr42 ();
    pub fn isr43 ();
    pub fn isr44 ();
    pub fn isr45 ();
    pub fn isr46 ();
    pub fn isr47 ();
    pub fn isr48 ();
    pub fn isr49 ();
    pub fn isr50 ();
    pub fn isr51 ();
    pub fn isr52 ();
    pub fn isr53 ();
    pub fn isr54 ();
    pub fn isr55 ();
    pub fn isr56 ();
    pub fn isr57 ();
    pub fn isr58 ();
    pub fn isr59 ();
    pub fn isr60 ();
    pub fn isr61 ();
    pub fn isr62 ();
    pub fn isr63 ();
    pub fn isr64 ();
    pub fn isr65 ();
    pub fn isr66 ();
    pub fn isr67 ();
    pub fn isr68 ();
    pub fn isr69 ();
    pub fn isr70 ();
    pub fn isr71 ();
    pub fn isr72 ();
    pub fn isr73 ();
    pub fn isr74 ();
    pub fn isr75 ();
    pub fn isr76 ();
    pub fn isr77 ();
    pub fn isr78 ();
    pub fn isr79 ();
    pub fn isr80 ();
    pub fn isr81 ();
    pub fn isr82 ();
    pub fn isr83 ();
    pub fn isr84 ();
    pub fn isr85 ();
    pub fn isr86 ();
    pub fn isr87 ();
    pub fn isr88 ();
    pub fn isr89 ();
    pub fn isr90 ();
    pub fn isr91 ();
    pub fn isr92 ();
    pub fn isr93 ();
    pub fn isr94 ();
    pub fn isr95 ();
    pub fn isr96 ();
    pub fn isr97 ();
    pub fn isr98 ();
    pub fn isr99 ();
    pub fn isr100 ();
    pub fn isr101 ();
    pub fn isr102 ();
    pub fn isr103 ();
    pub fn isr104 ();
    pub fn isr105 ();
    pub fn isr106 ();
    pub fn isr107 ();
    pub fn isr108 ();
    pub fn isr109 ();
    pub fn isr110 ();
    pub fn isr111 ();
    pub fn isr112 ();
    pub fn isr113 ();
    pub fn isr114 ();
    pub fn isr115 ();
    pub fn isr116 ();
    pub fn isr117 ();
    pub fn isr118 ();
    pub fn isr119 ();
    pub fn isr120 ();
    pub fn isr121 ();
    pub fn isr122 ();
    pub fn isr123 ();
    pub fn isr124 ();
    pub fn isr125 ();
    pub fn isr126 ();
    pub fn isr127 ();
    pub fn isr128 ();
    pub fn isr129 ();
    pub fn isr130 ();
    pub fn isr131 ();
    pub fn isr132 ();
    pub fn isr133 ();
    pub fn isr134 ();
    pub fn isr135 ();
    pub fn isr136 ();
    pub fn isr137 ();
    pub fn isr138 ();
    pub fn isr139 ();
    pub fn isr140 ();
    pub fn isr141 ();
    pub fn isr142 ();
    pub fn isr143 ();
    pub fn isr144 ();
    pub fn isr145 ();
    pub fn isr146 ();
    pub fn isr147 ();
    pub fn isr148 ();
    pub fn isr149 ();
    pub fn isr150 ();
    pub fn isr151 ();
    pub fn isr152 ();
    pub fn isr153 ();
    pub fn isr154 ();
    pub fn isr155 ();
    pub fn isr156 ();
    pub fn isr157 ();
    pub fn isr158 ();
    pub fn isr159 ();
    pub fn isr160 ();
    pub fn isr161 ();
    pub fn isr162 ();
    pub fn isr163 ();
    pub fn isr164 ();
    pub fn isr165 ();
    pub fn isr166 ();
    pub fn isr167 ();
    pub fn isr168 ();
    pub fn isr169 ();
    pub fn isr170 ();
    pub fn isr171 ();
    pub fn isr172 ();
    pub fn isr173 ();
    pub fn isr174 ();
    pub fn isr175 ();
    pub fn isr176 ();
    pub fn isr177 ();
    pub fn isr178 ();
    pub fn isr179 ();
    pub fn isr180 ();
    pub fn isr181 ();
    pub fn isr182 ();
    pub fn isr183 ();
    pub fn isr184 ();
    pub fn isr185 ();
    pub fn isr186 ();
    pub fn isr187 ();
    pub fn isr188 ();
    pub fn isr189 ();
    pub fn isr190 ();
    pub fn isr191 ();
    pub fn isr192 ();
    pub fn isr193 ();
    pub fn isr194 ();
    pub fn isr195 ();
    pub fn isr196 ();
    pub fn isr197 ();
    pub fn isr198 ();
    pub fn isr199 ();
    pub fn isr200 ();
    pub fn isr201 ();
    pub fn isr202 ();
    pub fn isr203 ();
    pub fn isr204 ();
    pub fn isr205 ();
    pub fn isr206 ();
    pub fn isr207 ();
    pub fn isr208 ();
    pub fn isr209 ();
    pub fn isr210 ();
    pub fn isr211 ();
    pub fn isr212 ();
    pub fn isr213 ();
    pub fn isr214 ();
    pub fn isr215 ();
    pub fn isr216 ();
    pub fn isr217 ();
    pub fn isr218 ();
    pub fn isr219 ();
    pub fn isr220 ();
    pub fn isr221 ();
    pub fn isr222 ();
    pub fn isr223 ();
    pub fn isr224 ();
    pub fn isr225 ();
    pub fn isr226 ();
    pub fn isr227 ();
    pub fn isr228 ();
    pub fn isr229 ();
    pub fn isr230 ();
    pub fn isr231 ();
    pub fn isr232 ();
    pub fn isr233 ();
    pub fn isr234 ();
    pub fn isr235 ();
    pub fn isr236 ();
    pub fn isr237 ();
    pub fn isr238 ();
    pub fn isr239 ();
    pub fn isr240 ();
    pub fn isr241 ();
    pub fn isr242 ();
    pub fn isr243 ();
    pub fn isr244 ();
    pub fn isr245 ();
    pub fn isr246 ();
    pub fn isr247 ();
    pub fn isr248 ();
    pub fn isr249 ();
    pub fn isr250 ();
    pub fn isr251 ();
    pub fn isr252 ();
    pub fn isr253 ();
    pub fn isr254 ();
}

pub fn initialize_interrupt_serive_routines() {
    set_idt_gate(
        0,
        isr0 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        1,
        isr1 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        2,
        isr2 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        3,
        isr3 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        4,
        isr4 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        5,
        isr5 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        6,
        isr6 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        7,
        isr7 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        8,
        isr8 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        9,
        isr9 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        10,
        isr10 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        11,
        isr11 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        12,
        isr12 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        13,
        isr13 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        14,
        isr14 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        15,
        isr15 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        16,
        isr16 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        17,
        isr17 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        18,
        isr18 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        19,
        isr19 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        20,
        isr20 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        21,
        isr21 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        22,
        isr22 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        23,
        isr23 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        24,
        isr24 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        25,
        isr25 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        26,
        isr26 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        27,
        isr27 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        28,
        isr28 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        29,
        isr29 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        30,
        isr30 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        31,
        isr31 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        32,
        isr32 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        33,
        isr33 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        34,
        isr34 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        35,
        isr35 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        36,
        isr36 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        37,
        isr37 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        38,
        isr38 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        39,
        isr39 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        40,
        isr40 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        41,
        isr41 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        42,
        isr42 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        43,
        isr43 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        44,
        isr44 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        45,
        isr45 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        46,
        isr46 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        47,
        isr47 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        48,
        isr48 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        49,
        isr49 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        50,
        isr50 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        51,
        isr51 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        52,
        isr52 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        53,
        isr53 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        54,
        isr54 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        55,
        isr55 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        56,
        isr56 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        57,
        isr57 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        58,
        isr58 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        59,
        isr59 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        60,
        isr60 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        61,
        isr61 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        62,
        isr62 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        63,
        isr63 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        64,
        isr64 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        65,
        isr65 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        66,
        isr66 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        67,
        isr67 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        68,
        isr68 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        69,
        isr69 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        70,
        isr70 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        71,
        isr71 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        72,
        isr72 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        73,
        isr73 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        74,
        isr74 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        75,
        isr75 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        76,
        isr76 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        77,
        isr77 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        78,
        isr78 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        79,
        isr79 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        80,
        isr80 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        81,
        isr81 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        82,
        isr82 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        83,
        isr83 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        84,
        isr84 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        85,
        isr85 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        86,
        isr86 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        87,
        isr87 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        88,
        isr88 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        89,
        isr89 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        90,
        isr90 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        91,
        isr91 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        92,
        isr92 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        93,
        isr93 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        94,
        isr94 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        95,
        isr95 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        96,
        isr96 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        97,
        isr97 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        98,
        isr98 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        99,
        isr99 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        100,
        isr100 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        101,
        isr101 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        102,
        isr102 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        103,
        isr103 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        104,
        isr104 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        105,
        isr105 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        106,
        isr106 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        107,
        isr107 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        108,
        isr108 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        109,
        isr109 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        110,
        isr110 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        111,
        isr111 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        112,
        isr112 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        113,
        isr113 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        114,
        isr114 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        115,
        isr115 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        116,
        isr116 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        117,
        isr117 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        118,
        isr118 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        119,
        isr119 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        120,
        isr120 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        121,
        isr121 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        122,
        isr122 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        123,
        isr123 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        124,
        isr124 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        125,
        isr125 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        126,
        isr126 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        127,
        isr127 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        128,
        isr128 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        129,
        isr129 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        130,
        isr130 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        131,
        isr131 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        132,
        isr132 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        133,
        isr133 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        134,
        isr134 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        135,
        isr135 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        136,
        isr136 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        137,
        isr137 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        138,
        isr138 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        139,
        isr139 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        140,
        isr140 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        141,
        isr141 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        142,
        isr142 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        143,
        isr143 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        144,
        isr144 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        145,
        isr145 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        146,
        isr146 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        147,
        isr147 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        148,
        isr148 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        149,
        isr149 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        150,
        isr150 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        151,
        isr151 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        152,
        isr152 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        153,
        isr153 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        154,
        isr154 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        155,
        isr155 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        156,
        isr156 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        157,
        isr157 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        158,
        isr158 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        159,
        isr159 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        160,
        isr160 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        161,
        isr161 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        162,
        isr162 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        163,
        isr163 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        164,
        isr164 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        165,
        isr165 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        166,
        isr166 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        167,
        isr167 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        168,
        isr168 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        169,
        isr169 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        170,
        isr170 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        171,
        isr171 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        172,
        isr172 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        173,
        isr173 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        174,
        isr174 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        175,
        isr175 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        176,
        isr176 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        177,
        isr177 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        178,
        isr178 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        179,
        isr179 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        180,
        isr180 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        181,
        isr181 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        182,
        isr182 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        183,
        isr183 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        184,
        isr184 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        185,
        isr185 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        186,
        isr186 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        187,
        isr187 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        188,
        isr188 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        189,
        isr189 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        190,
        isr190 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        191,
        isr191 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        192,
        isr192 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        193,
        isr193 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        194,
        isr194 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        195,
        isr195 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        196,
        isr196 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        197,
        isr197 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        198,
        isr198 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        199,
        isr199 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        200,
        isr200 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        201,
        isr201 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        202,
        isr202 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        203,
        isr203 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        204,
        isr204 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        205,
        isr205 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        206,
        isr206 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        207,
        isr207 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        208,
        isr208 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        209,
        isr209 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        210,
        isr210 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        211,
        isr211 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        212,
        isr212 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        213,
        isr213 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        214,
        isr214 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        215,
        isr215 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        216,
        isr216 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        217,
        isr217 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        218,
        isr218 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        219,
        isr219 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        220,
        isr220 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        221,
        isr221 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        222,
        isr222 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        223,
        isr223 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        224,
        isr224 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        225,
        isr225 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        226,
        isr226 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        227,
        isr227 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        228,
        isr228 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        229,
        isr229 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        230,
        isr230 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        231,
        isr231 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        232,
        isr232 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        233,
        isr233 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        234,
        isr234 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        235,
        isr235 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        236,
        isr236 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        237,
        isr237 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        238,
        isr238 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        239,
        isr239 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        240,
        isr240 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        241,
        isr241 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        242,
        isr242 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        243,
        isr243 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        244,
        isr244 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        245,
        isr245 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        246,
        isr246 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        247,
        isr247 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        248,
        isr248 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        249,
        isr249 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        250,
        isr250 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        251,
        isr251 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        252,
        isr252 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        253,
        isr253 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
    set_idt_gate(
        254,
        isr254 as *const (),
        0x08,
        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,
    );
}
