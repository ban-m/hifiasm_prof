# HiFiAsm Phasecheck

Author: Bansho Masutani<ban-m@g.ecc.u-tokyo.ac.jp>
Date: 2022/05/11

# Summary


HiFiAsm is an assembler for PacBio's HiFi datasets. It can detect SNP/SVs in its algorithm, phase these variants, and produce a phased assembly. Additionally, it can handle other datasets, such as Trio datasets and Hi-C datasets to produce more "phased" assembly graphs.


There are -- at least -- three types of assembly graphs produced by HiFiAsm. One is a primary unitig, named `p_utg`, and the others are phased assemblies, named `hap1.ctg` and `hap2.ctg`. If you visualize these three, most of the branches in the primary unitig seem solved in the last two.

However, are these choices correct? By manual investigation, it seems that the homozygous regions in the primary contig are longer than the long reads(i.e., longer than 25Kbp).  So, theoretically, it *can not* be solved by the reads unless the assembler surveys the homozygous region again to find useful variants hidden in these contigs.


In this crate, I check whether these phasings are correct or not. In other words, I create artificial diploid genomes with very small divergence rates. If I simulate HiFi reads from these genomes, HiFiAsm should produce primary unitigs containing lots of branches. Also, if my observation is correct, it produces "phased" contigs with very small bubbles, too.

By checking whether the nucleotides on these phased contigs are concordant with the correct genomes, I can estimate the phasing accuracy. In other words, if the phasing procedure can --  somehow I do not know --  solves the phasing problem with reads shorter than the homozygous regions, the assemblies perfectly match one of the true genomes. Otherwise, the variants are chosen randomly, producing around 50% accuracy. 

I hypothesize that the accuracy would be among 50-90% as the assembler can be conservative in the primary assembly and aggressive in the phasing step.

# Synopsis

```
bash ./script/run.sh
```

# Requirements

- Rust and Cargo(>1.60)
- HiFiAsm
- Minimap2
