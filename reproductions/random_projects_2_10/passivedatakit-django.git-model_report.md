# Model report for file:///tmp/top-repos-quality-repos-bsx51bvb/passivedatakit-django.git HEAD e5db20d26d8b45d3288c787cae85e3c2bf4b80a7

### Dump

```json
{'created_at': '2021-08-22 06:20:24',
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
 'size': '20.2 kB',
 'tags': [],
 'uuid': '78a6ac91-4015-4488-af5b-929420279b8d',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-bsx51bvb/passivedatakit-django.git e5db20d26d8b45d3288c787cae85e3c2bf4b80a7

# javascript
43 rules, avg.len. 9.3
## train
PPCR: 0.875329
### report
macro
{'f1-score': 0.7180481578151084,
 'precision': 0.805612784293144,
 'recall': 0.6715372539247656,
 'support': 62158}
micro
{'f1-score': 0.970253225650761,
 'precision': 0.970253225650761,
 'recall': 0.970253225650761,
 'support': 62158}
weighted
{'f1-score': 0.967522925983605,
 'precision': 0.9681857891448288,
 'recall': 0.970253225650761,
 'support': 62158}
### report_full
macro
{'f1-score': 0.506062455054016,
 'precision': 0.805612784293144,
 'recall': 0.42505583422192605,
 'support': 71011}
micro
{'f1-score': 0.9057513385247317,
 'precision': 0.970253225650761,
 'recall': 0.8492909549224767,
 'support': 71011}
weighted
{'f1-score': 0.8710693695717873,
 'precision': 0.9449503258536148,
 'recall': 0.8492909549224767,
 'support': 71011}
## test
PPCR: 0.911523
### report
macro
{'f1-score': 0.5114157195673936,
 'precision': 0.5832070846942519,
 'recall': 0.5120387134854703,
 'support': 22418}
micro
{'f1-score': 0.9150682487287002,
 'precision': 0.9150682487287002,
 'recall': 0.9150682487287002,
 'support': 22418}
weighted
{'f1-score': 0.907157930801154,
 'precision': 0.9122848951736798,
 'recall': 0.9150682487287002,
 'support': 22418}
### report_full
macro
{'f1-score': 0.4020863490146553,
 'precision': 0.5832070846942519,
 'recall': 0.3590752894589577,
 'support': 24594}
micro
{'f1-score': 0.872713349783034,
 'precision': 0.9150682487287002,
 'recall': 0.8341058794828007,
 'support': 24594}
weighted
{'f1-score': 0.8470295921793397,
 'precision': 0.8996679470598821,
 'recall': 0.8341058794828007,
 'support': 24594}
```

## javascript
### Summary
26 rules, avg.len. 9.3

| | |
|-|-|
|Min support|96|
|Max support|10397|
|Min confidence|0.9200743436813354|
|Max confidence|0.999515950679779|

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
| 1 | `  -1.roles in {STRING}<br>	∧ -2.diff_offset ≤ 9<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.972. Support: 127.` |
| 2 | `  -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 10397.` |
| 3 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {), ]}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.946. Support: 101.` |
| 4 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<space>}<br>	∧ +1.reserved = (<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 99.` |
| 5 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -4.diff_offset ≥ 7<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 5213.` |
| 6 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ -3.length ≥ 2<br>	∧ -4.diff_offset ≤ 6<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 96.` |
| 7 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.length ≤ 1<br>	∧ -4.diff_offset ≤ 6<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 938.` |
| 8 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1033.` |
| 9 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 7229.` |
| 10 | `  -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 3837.` |
| 11 | `  -1.reserved not in {(}<br>	∧ -1.roles in {STRING} and not in {IDENTIFIER}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.964. Support: 208.` |
| 12 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 2420.` |
| 13 | `  •••start_line ≥ 210<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles in {STRING} and not in {IDENTIFIER}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.953. Support: 1215.` |
| 14 | `  -1.reserved = ;<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles in {EXPRESSION} and not in {ARGUMENT}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.929. Support: 248.` |
| 15 | `  -1.reserved = ;<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.reserved not in {;, }}<br>	∧ ^1.roles in {FOR} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 98.` |
| 16 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.946. Support: 990.` |
| 17 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 772.` |
| 18 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.reserved not in {), ,, ;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 727.` |
| 19 | `  -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -2.label in {<-space>}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.987. Support: 112.` |
| 20 | `  -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.reserved = else<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 157.` |
| 21 | `  •••start_col ≥ 23<br>	∧ -1.diff_col ≥ 9<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.reserved not in {), ,}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 165.` |
| 22 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ]<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.reserved not in {), ,}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.920. Support: 269.` |
| 23 | `  •••start_col ≥ 6<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 131.` |
| 24 | `  •••start_col ≥ 8<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, ], {, }}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -3.label not in {<newline>}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED, SWITCH}<br>⇒ y = ␣<br>Confidence: 0.961. Support: 7755.` |
| 25 | `  •••start_col ≤ 7<br>	∧ •••start_line ≥ 247<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, ], {, }}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -3.label not in {<newline>}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED, SWITCH}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 270.` |
| 26 | `  •••start_col ≤ 5<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, ], {, }}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.reserved not in {), ,}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 226.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 9.307692307692308, "max_conf": 0.999515950679779, "max_support": 10397, "min_conf": 0.9200743436813354, "min_support": 96, "num_rules": 26}}
```
</details>
