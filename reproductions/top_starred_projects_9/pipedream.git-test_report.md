# Test report for javascript / file:///tmp/top-repos-quality-repos-a0qleu_g/pipedream.git HEAD 2ad7b3c7c44cb0262e31151a182044fe578026fc

### Classification report

PPCR: 0.968

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.977| 0.998| 0.986| 0.987| 0.981| 17960| 18177| 0.988 |
| `␣` | 0.987| 0.977| 0.935| 0.982| 0.960| 8840| 9242| 0.957 |
| `⏎` | 0.938| 0.977| 0.912| 0.957| 0.925| 3261| 3495| 0.933 |
| `"` | 0.975| 1.000| 0.969| 0.987| 0.972| 3194| 3296| 0.969 |
| `⏎␣⁻␣⁻` | 0.981| 0.930| 0.905| 0.954| 0.941| 1789| 1837| 0.974 |
| `⏎␣⁺␣⁺` | 0.977| 0.872| 0.801| 0.921| 0.880| 1740| 1893| 0.919 |
| `⏎⏎` | 1.000| 0.470| 0.380| 0.639| 0.551| 266| 329| 0.809 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 77| 98| 0.786 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 25| 25| 1.000 |
| `weighted avg` | 0.973| 0.976| 0.944| 0.974| 0.957| 37152| 38392| 0.968 |
| `micro avg` | 0.976| 0.976| 0.944| 0.976| 0.960| 37152| 38392| 0.968 |
| `macro avg` | 0.759| 0.692| 0.654| 0.714| 0.690| 37152| 38392| 0.968 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| '| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|217 |17925 |32 |0 |0 |1 |2 |0 |0 |0 |
|402 |95 |8641 |65 |0 |33 |6 |0 |0 |0 |
|234 |61 |10 |3187 |0 |2 |1 |0 |0 |0 |
|102 |0 |1 |0 |3193 |0 |0 |0 |0 |0 |
|153 |151 |64 |2 |6 |1517 |0 |0 |0 |0 |
|48 |88 |5 |33 |0 |0 |1663 |0 |0 |0 |
|63 |31 |0 |110 |0 |0 |0 |125 |0 |0 |
|21 |0 |0 |0 |77 |0 |0 |0 |0 |0 |
|0 |0 |0 |1 |0 |0 |24 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9870170015455951, "precision": 0.9746642246642246, "recall": 0.9996869129618033, "support": 3194}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 77}, "macro avg": {"f1-score": 0.7143283133736625, "precision": 0.7593245311613325, "recall": 0.6915408760548085, "support": 37152}, "micro avg": {"f1-score": 0.9757482773471146, "precision": 0.9757482773471146, "recall": 0.9757482773471146, "support": 37152}, "weighted avg": {"f1-score": 0.9735755382158566, "precision": 0.9733366618785846, "recall": 0.9757482773471146, "support": 37152}, "\u2205": {"f1-score": 0.9873041227176338, "precision": 0.9767860062121955, "recall": 0.9980512249443207, "support": 17960}, "\u23ce": {"f1-score": 0.9572007808980327, "precision": 0.9379046497939965, "recall": 0.9773075743636921, "support": 3261}, "\u23ce\u23ce": {"f1-score": 0.639386189258312, "precision": 1.0, "recall": 0.4699248120300752, "support": 266}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9213483146067417, "precision": 0.9768190598840953, "recall": 0.8718390804597701, "support": 1740}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9543758967001434, "precision": 0.9805424528301887, "recall": 0.9295695919508105, "support": 1789}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 25}, "\u2423": {"f1-score": 0.9823225146365031, "precision": 0.9872043870672912, "recall": 0.9774886877828054, "support": 8840}},
  "cl_report_full": {"\"": {"f1-score": 0.9716981132075472, "precision": 0.9746642246642246, "recall": 0.96875, "support": 3296}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 98}, "macro avg": {"f1-score": 0.6900813963176828, "precision": 0.7593245311613325, "recall": 0.6542582507670515, "support": 38392}, "micro avg": {"f1-score": 0.9597320766705496, "precision": 0.9757482773471146, "recall": 0.9442331735778288, "support": 38392}, "weighted avg": {"f1-score": 0.9566369087879059, "precision": 0.9728231710233365, "recall": 0.9442331735778288, "support": 38392}, "\u2205": {"f1-score": 0.9814388961892246, "precision": 0.9767860062121955, "recall": 0.9861363261264235, "support": 18177}, "\u23ce": {"f1-score": 0.9247062237052082, "precision": 0.9379046497939965, "recall": 0.9118741058655222, "support": 3495}, "\u23ce\u23ce": {"f1-score": 0.5506607929515419, "precision": 1.0, "recall": 0.3799392097264438, "support": 329}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8804410911201394, "precision": 0.9768190598840953, "recall": 0.8013734812466984, "support": 1893}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9414095669402773, "precision": 0.9805424528301887, "recall": 0.9052803483941209, "support": 1837}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 25}, "\u2423": {"f1-score": 0.960377882745207, "precision": 0.9872043870672912, "recall": 0.9349707855442545, "support": 9242}},
  "ppcr": 0.9677016045009377
}
```
</details>