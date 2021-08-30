# Model report for file:///tmp/top-repos-quality-repos-m3c6u3tn/realm-js.git HEAD ae82ff7263680cba4a015dbf16d2d5a95d17e3cb

### Dump

```json
{'created_at': '2021-08-24 20:46:32',
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
 'size': '19.6 kB',
 'tags': [],
 'uuid': '68a48a61-7890-4a21-8796-94610df9b104',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-m3c6u3tn/realm-js.git ae82ff7263680cba4a015dbf16d2d5a95d17e3cb

# javascript
41 rules, avg.len. 8.4
## train
PPCR: 0.896714
### report
macro
{'f1-score': 0.6313025918716397,
 'precision': 0.631665723888353,
 'recall': 0.6339894553079867,
 'support': 78015}
micro
{'f1-score': 0.9571749022623854,
 'precision': 0.9571749022623854,
 'recall': 0.9571749022623854,
 'support': 78015}
weighted
{'f1-score': 0.9519094215459097,
 'precision': 0.9473231205575381,
 'recall': 0.9571749022623854,
 'support': 78015}
### report_full
macro
{'f1-score': 0.5431716765517125,
 'precision': 0.631665723888353,
 'recall': 0.4966489046043206,
 'support': 87001}
micro
{'f1-score': 0.9050516313569594,
 'precision': 0.9571749022623854,
 'recall': 0.8583119734255928,
 'support': 87001}
weighted
{'f1-score': 0.8885739289582262,
 'precision': 0.9332709971121946,
 'recall': 0.8583119734255928,
 'support': 87001}
## test
PPCR: 0.888328
### report
macro
{'f1-score': 0.6399535623356669,
 'precision': 0.6447702210747634,
 'recall': 0.6374290536758151,
 'support': 21478}
micro
{'f1-score': 0.9619145171803706,
 'precision': 0.9619145171803706,
 'recall': 0.9619145171803706,
 'support': 21478}
weighted
{'f1-score': 0.9580177388549684,
 'precision': 0.9545384271529287,
 'recall': 0.9619145171803706,
 'support': 21478}
### report_full
macro
{'f1-score': 0.5457366333713551,
 'precision': 0.6447702210747634,
 'recall': 0.4958518604550834,
 'support': 24178}
micro
{'f1-score': 0.9050289118626248,
 'precision': 0.9619145171803706,
 'recall': 0.8544958226486888,
 'support': 24178}
weighted
{'f1-score': 0.8905963768824661,
 'precision': 0.9419727888616376,
 'recall': 0.8544958226486888,
 'support': 24178}
```

## javascript
### Summary
20 rules, avg.len. 8.0

| | |
|-|-|
|Min support|121|
|Max support|12620|
|Min confidence|0.9315165877342224|
|Max confidence|0.9984720349311829|

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
| 1 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 12620.` |
| 2 | `  -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 2799.` |
| 3 | `  •••start_col ≥ 74<br>	∧ -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.length ≥ 72<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {FILE} and not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.955. Support: 146.` |
| 4 | `  •••start_col ≤ 74<br>	∧ -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.length ≥ 72<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {FILE} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 122.` |
| 5 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.length ≤ 71<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {FILE} and not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.974. Support: 1233.` |
| 6 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.length ≥ 18<br>	∧ ^1.roles in {FILE} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 1253.` |
| 7 | `  -1.diff_offset ≥ 12<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.length ≤ 17<br>	∧ ^1.roles in {FILE} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 191.` |
| 8 | `  -1.diff_col ≥ 2<br>	∧ -1.diff_offset ≤ 12<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -2.roles not in {COMMENT}<br>	∧ +1.length ≤ 17<br>	∧ +2.length ≥ 18<br>	∧ ^1.roles in {FILE} and not in {IDENTIFIER}<br>⇒ y = ⏎⏎<br>Confidence: 0.996. Support: 126.` |
| 9 | `  -1.diff_offset ≤ 12<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -2.roles not in {COMMENT}<br>	∧ +1.length ≤ 17<br>	∧ +2.length ≤ 17<br>	∧ ^1.roles in {FILE} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 185.` |
| 10 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.length ≤ 13<br>	∧ ^1.roles in {LITERAL} and not in {FILE, IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.963. Support: 121.` |
| 11 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL, STRING} and not in {FILE, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 381.` |
| 12 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label in {<-space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, IDENTIFIER, LITERAL, STATEMENT}<br>⇒ y = ⏎⏎<br>Confidence: 0.954. Support: 184.` |
| 13 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -2.label not in {<-space>}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.947. Support: 6857.` |
| 14 | `  +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 2210.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 182.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.932. Support: 2110.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 279.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 261.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -3.diff_col ≥ 2<br>	∧ -4.roles not in {COMMENT}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 11453.` |
| 20 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_col ≥ 2<br>	∧ -4.roles not in {COMMENT}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 7810.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.95, "max_conf": 0.9984720349311829, "max_support": 12620, "min_conf": 0.9315165877342224, "min_support": 121, "num_rules": 20}}
```
</details>