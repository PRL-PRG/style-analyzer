# Model report for file:///tmp/top-repos-quality-repos-ui2t_z_7/afm-client.git HEAD a10124ad68b41b31b81dd5e4e3e4d404f58f956d

### Dump

```json
{'created_at': '2021-08-22 05:10:59',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-81-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '18.3 kB',
 'tags': [],
 'uuid': 'b1c3e9ca-ce0f-4155-9229-ec1399f4767b',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ui2t_z_7/afm-client.git a10124ad68b41b31b81dd5e4e3e4d404f58f956d

# javascript
23 rules, avg.len. 6.6
## train
PPCR: 0.856514
### report
macro
{'f1-score': 0.6402368355813599,
 'precision': 0.6465192743723657,
 'recall': 0.6353615670338788,
 'support': 18493}
micro
{'f1-score': 0.9644730438544314,
 'precision': 0.9644730438544314,
 'recall': 0.9644730438544314,
 'support': 18493}
weighted
{'f1-score': 0.9609282516095295,
 'precision': 0.9579019478158676,
 'recall': 0.9644730438544314,
 'support': 18493}
### report_full
macro
{'f1-score': 0.49402713732837567,
 'precision': 0.6465192743723657,
 'recall': 0.4211464704561038,
 'support': 21591}
micro
{'f1-score': 0.8899311445963476,
 'precision': 0.9644730438544314,
 'recall': 0.8260849428002408,
 'support': 21591}
weighted
{'f1-score': 0.8610032655038838,
 'precision': 0.9182017447727581,
 'recall': 0.8260849428002408,
 'support': 21591}
## test
PPCR: 0.768116
### report
macro
{'f1-score': 0.19240362811791384,
 'precision': 0.18947368421052632,
 'recall': 0.196,
 'support': 53}
micro
{'f1-score': 0.7735849056603775,
 'precision': 0.7735849056603774,
 'recall': 0.7735849056603774,
 'support': 53}
weighted
{'f1-score': 0.765006631583451,
 'precision': 0.7586891757696127,
 'recall': 0.7735849056603774,
 'support': 53}
### report_full
macro
{'f1-score': 0.18856209150326797,
 'precision': 0.18947368421052632,
 'recall': 0.18888888888888888,
 'support': 69}
micro
{'f1-score': 0.6721311475409837,
 'precision': 0.7735849056603774,
 'recall': 0.5942028985507246,
 'support': 69}
weighted
{'f1-score': 0.6009756559628682,
 'precision': 0.6117467581998475,
 'recall': 0.5942028985507246,
 'support': 69}
```

## javascript
### Summary
13 rules, avg.len. 5.5

| | |
|-|-|
|Min support|108|
|Max support|4173|
|Min confidence|0.9290540814399719|
|Max confidence|0.9992559552192688|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 90,
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.reserved = (<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 137.` |
| 2 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {LITERAL}<br>	∧ +2.roles in {EXPRESSION} and not in {FUNCTION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 899.` |
| 3 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.label in {<space>}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +2.roles not in {EXPRESSION, FUNCTION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 775.` |
| 4 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.label not in {<space>}<br>	∧ +2.roles not in {EXPRESSION, FUNCTION}<br>	∧ +2.length ≥ 4<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 108.` |
| 5 | `  -1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.998. Support: 273.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 4173.` |
| 7 | `  -1.internal_type = CommentLine<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.977. Support: 582.` |
| 8 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 1209.` |
| 9 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 1087.` |
| 10 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 672.` |
| 11 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 612.` |
| 12 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.length ≥ 18<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {BLOCK, OPERATOR}<br>⇒ y = ⏎⏎<br>Confidence: 0.929. Support: 148.` |
| 13 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 214.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.461538461538462, "max_conf": 0.9992559552192688, "max_support": 4173, "min_conf": 0.9290540814399719, "min_support": 108, "num_rules": 13}}
```
</details>
