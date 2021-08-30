# Model report for file:///tmp/top-repos-quality-repos-f_a16m61/node-ar-drone.git HEAD 11667d4640a55111a6ad0206336685e69c7323fb

### Dump

```json
{'created_at': '2021-08-29 16:08:18',
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
 'size': '19.1 kB',
 'tags': [],
 'uuid': 'd1dc1654-9ecc-4256-9f74-af33b3e1f0b6',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-f_a16m61/node-ar-drone.git 11667d4640a55111a6ad0206336685e69c7323fb

# javascript
19 rules, avg.len. 5.8
## train
PPCR: 0.940269
### report
macro
{'f1-score': 0.6407687743799528,
 'precision': 0.6585613519194936,
 'recall': 0.6274085869588428,
 'support': 25612}
micro
{'f1-score': 0.9552553490551304,
 'precision': 0.9552553490551304,
 'recall': 0.9552553490551304,
 'support': 25612}
weighted
{'f1-score': 0.9501744812921299,
 'precision': 0.9464970786931303,
 'recall': 0.9552553490551304,
 'support': 25612}
### report_full
macro
{'f1-score': 0.5709885557115644,
 'precision': 0.6585613519194936,
 'recall': 0.5399179052315742,
 'support': 27239}
micro
{'f1-score': 0.9258481391080585,
 'precision': 0.9552553490551304,
 'recall': 0.8981974374977055,
 'support': 27239}
weighted
{'f1-score': 0.9092129256147768,
 'precision': 0.9463754775297958,
 'recall': 0.8981974374977055,
 'support': 27239}
## test
PPCR: 0.950319
### report
macro
{'f1-score': 0.6278561344406657,
 'precision': 0.6331073063377923,
 'recall': 0.6300051767553212,
 'support': 7154}
micro
{'f1-score': 0.9470226446743081,
 'precision': 0.9470226446743081,
 'recall': 0.9470226446743081,
 'support': 7154}
weighted
{'f1-score': 0.944075311969135,
 'precision': 0.9429703635067213,
 'recall': 0.9470226446743081,
 'support': 7154}
### report_full
macro
{'f1-score': 0.5748945661129796,
 'precision': 0.6331073063377923,
 'recall': 0.5594801959560364,
 'support': 7528}
micro
{'f1-score': 0.9228987876311129,
 'precision': 0.9470226446743081,
 'recall': 0.8999734325185972,
 'support': 7528}
weighted
{'f1-score': 0.9078827049839214,
 'precision': 0.9417564227047185,
 'recall': 0.8999734325185972,
 'support': 7528}
```

## javascript
### Summary
14 rules, avg.len. 6.0

| | |
|-|-|
|Min support|92|
|Max support|6366|
|Min confidence|0.9228110313415527|
|Max confidence|0.9991909265518188|

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
                     'max_depth': 10,
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
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.991. Support: 5954.` |
| 2 | `  -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.998. Support: 311.` |
| 3 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 774.` |
| 4 | `  -1.reserved not in {(, ;, {}<br>	∧ -3.label in {<-space>}<br>	∧ +1.roles in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ +3.reserved = function<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎⏎<br>Confidence: 0.995. Support: 92.` |
| 5 | `  -1.reserved not in {(, ;, {}<br>	∧ -3.label not in {<-space>}<br>	∧ +1.roles in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ +3.reserved = function<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.995. Support: 100.` |
| 6 | `  -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ +3.reserved not in {function}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.959. Support: 376.` |
| 7 | `  -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {COMMENT} and not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 281.` |
| 8 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.997. Support: 176.` |
| 9 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles in {COMMENT}<br>	∧ -4.diff_line ≥ 1<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.931. Support: 108.` |
| 10 | `  -1.internal_type = StringLiteral<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.999. Support: 618.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 470.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.923. Support: 434.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.954. Support: 97.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.954. Support: 6366.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.0, "max_conf": 0.9991909265518188, "max_support": 6366, "min_conf": 0.9228110313415527, "min_support": 92, "num_rules": 14}}
```
</details>
