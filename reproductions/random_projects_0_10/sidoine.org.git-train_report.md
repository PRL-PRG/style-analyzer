# Train report for javascript / file:///tmp/top-repos-quality-repos-l_vk29hn/sidoine.org.git HEAD ed247b93ac65ff3f878ddf81ccacff09282ed1b1

### Classification report

PPCR: 0.380

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.956| 1.000| 0.556| 0.977| 0.703| 1254| 2256| 0.556 |
| `"` | 1.000| 1.000| 0.500| 1.000| 0.667| 161| 322| 0.500 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 32| 114| 0.281 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 882| 0.018 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 109| 0.083 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 196| 0.005 |
| `micro avg` | 0.961| 0.961| 0.365| 0.961| 0.529| 1473| 3879| 0.380 |
| `weighted avg` | 0.923| 0.961| 0.365| 0.941| 0.464| 1473| 3879| 0.380 |
| `macro avg` | 0.326| 0.333| 0.176| 0.330| 0.228| 1473| 3879| 0.380 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|1002 |1254 |0 |0 |0 |0 |0 |
|866 |16 |0 |0 |0 |0 |0 |
|161 |0 |0 |161 |0 |0 |0 |
|195 |1 |0 |0 |0 |0 |0 |
|82 |32 |0 |0 |0 |0 |0 |
|100 |9 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/components/bio.js | 8 |
| src/pages/private.js | 7 |
| src/pages/blog.js | 7 |
| src/templates/blog-post.js | 7 |
| src/pages/index.js | 7 |
| src/components/Footer.js | 5 |
| src/components/Title.js | 4 |
| src/components/Cards.js | 4 |
| src/components/ProjectListing.js | 3 |
| src/components/Seo.js | 3 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 161}, "macro avg": {"f1-score": 0.3295661210704079, "precision": 0.32596544715447157, "recall": 0.3333333333333333, "support": 1473}, "micro avg": {"f1-score": 0.9606245756958588, "precision": 0.9606245756958588, "recall": 0.9606245756958588, "support": 1473}, "weighted avg": {"f1-score": 0.9413818702876775, "precision": 0.9229898332588214, "recall": 0.9606245756958588, "support": 1473}, "\u2205": {"f1-score": 0.9773967264224473, "precision": 0.9557926829268293, "recall": 1.0, "support": 1254}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}},
  "cl_report_full": {"\"": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 322}, "macro avg": {"f1-score": 0.22826357747882411, "precision": 0.32596544715447157, "recall": 0.17597517730496456, "support": 3879}, "micro avg": {"f1-score": 0.5287742899850523, "precision": 0.9606245756958588, "recall": 0.3647847383346223, "support": 3879}, "weighted avg": {"f1-score": 0.46415118623872903, "precision": 0.6388936047133094, "recall": 0.3647847383346223, "support": 3879}, "\u2205": {"f1-score": 0.702914798206278, "precision": 0.9557926829268293, "recall": 0.5558510638297872, "support": 2256}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 196}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 109}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 114}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 882}},
  "ppcr": 0.37973704563031707
}
```
</details>
