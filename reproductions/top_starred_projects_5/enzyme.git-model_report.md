# Model report for file:///tmp/top-repos-quality-repos-kkd3jyau/enzyme.git HEAD 8c55734eb19a2f7555eb3d695ae11401fa1f9847

### Dump

```json
{'created_at': '2021-08-29 23:50:29',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.15.0-135-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '16.6 kB',
 'tags': [],
 'uuid': '9354d9ae-9004-458b-a63b-fd4e44313b6b',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-kkd3jyau/enzyme.git 8c55734eb19a2f7555eb3d695ae11401fa1f9847

# javascript
24 rules, avg.len. 7.5
## train
PPCR: 0.905063
### report
macro
{'f1-score': 0.940750735135241,
 'precision': 0.9560525918399307,
 'recall': 0.9269363186603855,
 'support': 31889}
micro
{'f1-score': 0.9640628429866098,
 'precision': 0.9640628429866098,
 'recall': 0.9640628429866098,
 'support': 31889}
weighted
{'f1-score': 0.9637664691225597,
 'precision': 0.9639679187353695,
 'recall': 0.9640628429866098,
 'support': 31889}
### report_full
macro
{'f1-score': 0.8429027760880983,
 'precision': 0.9560525918399307,
 'recall': 0.7738316552079091,
 'support': 35234}
micro
{'f1-score': 0.9160198441666791,
 'precision': 0.9640628429866098,
 'recall': 0.872537889538514,
 'support': 35234}
weighted
{'f1-score': 0.9086408387007705,
 'precision': 0.9611071064084499,
 'recall': 0.872537889538514,
 'support': 35234}
## test
PPCR: 0.919838
### report
macro
{'f1-score': 0.9392421178373683,
 'precision': 0.9650752932058758,
 'recall': 0.9203002957547757,
 'support': 3408}
micro
{'f1-score': 0.9518779342723005,
 'precision': 0.9518779342723005,
 'recall': 0.9518779342723005,
 'support': 3408}
weighted
{'f1-score': 0.950711328779799,
 'precision': 0.953346005975377,
 'recall': 0.9518779342723005,
 'support': 3408}
### report_full
macro
{'f1-score': 0.8012363532223442,
 'precision': 0.9650752932058758,
 'recall': 0.7298282084954957,
 'support': 3705}
micro
{'f1-score': 0.912132714747645,
 'precision': 0.9518779342723005,
 'recall': 0.8755735492577598,
 'support': 3705}
weighted
{'f1-score': 0.9009566427176684,
 'precision': 0.9535888059469224,
 'recall': 0.8755735492577598,
 'support': 3705}
```

## javascript
### Summary
20 rules, avg.len. 7.1

| | |
|-|-|
|Min support|92|
|Max support|5040|
|Min confidence|0.929347813129425|
|Max confidence|0.9992774724960327|

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
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.985. Support: 3554.` |
| 2 | `  -1.reserved = (<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.998. Support: 233.` |
| 3 | `  -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 1565.` |
| 4 | `  -1.reserved = {<br>	∧ -3.diff_offset ≤ 6<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.950. Support: 954.` |
| 5 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.999. Support: 444.` |
| 6 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {FILE}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 397.` |
| 7 | `  -1.internal_type = CommentBlock<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {FILE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 173.` |
| 8 | `  -1.internal_type not in {CommentBlock}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.diff_offset ≥ 19<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {FILE}<br>⇒ y = ⏎<br>Confidence: 0.977. Support: 365.` |
| 9 | `  •••start_col ≤ 5<br>	∧ -1.internal_type not in {CommentBlock}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.diff_offset ≤ 18<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {FILE}<br>⇒ y = ⏎⏎<br>Confidence: 0.939. Support: 173.` |
| 10 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label in {<-space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE, LITERAL, STATEMENT}<br>⇒ y = ⏎⏎<br>Confidence: 0.968. Support: 139.` |
| 11 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.roles in {IMPORT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE, LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.929. Support: 92.` |
| 12 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.roles not in {IMPORT}<br>	∧ -3.diff_offset ≥ 6<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.940. Support: 4125.` |
| 13 | `  -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.972. Support: 1039.` |
| 14 | `  +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 1135.` |
| 15 | `  +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 873.` |
| 16 | `  -1.internal_type = StringLiteral<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.999. Support: 692.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 375.` |
| 18 | `  -1.internal_type = Identifier<br>	∧ -1.reserved not in {,, if}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {CONDITION}<br>	∧ ^2.internal_type = IfStatement<br>⇒ y = ∅<br>Confidence: 0.943. Support: 97.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 5040.` |
| 20 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.934. Support: 2805.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.1, "max_conf": 0.9992774724960327, "max_support": 5040, "min_conf": 0.929347813129425, "min_support": 92, "num_rules": 20}}
```
</details>
