# Model report for file:///tmp/top-repos-quality-repos-ys7mc6mz/oauthd.git HEAD 2d8819d3c223eafa38f11c028db84b4162133b26

### Dump

```json
{'created_at': '2021-08-29 20:50:46',
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
 'size': '17.9 kB',
 'tags': [],
 'uuid': '2d4ba577-d9f4-49ca-a4f6-6f875f624f87',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ys7mc6mz/oauthd.git 2d8819d3c223eafa38f11c028db84b4162133b26

# javascript
64 rules, avg.len. 5.4
## train
PPCR: 0.943856
### report
macro
{'f1-score': 0.6550439033293999,
 'precision': 0.6689015145567538,
 'recall': 0.6448414127140636,
 'support': 7666}
micro
{'f1-score': 0.8905557004956953,
 'precision': 0.8905557004956953,
 'recall': 0.8905557004956953,
 'support': 7666}
weighted
{'f1-score': 0.8817340352136809,
 'precision': 0.8774695129249231,
 'recall': 0.8905557004956953,
 'support': 7666}
### report_full
macro
{'f1-score': 0.62423195711872,
 'precision': 0.6689015145567538,
 'recall': 0.5988216788530091,
 'support': 8122}
micro
{'f1-score': 0.8648340511781101,
 'precision': 0.8905557004956953,
 'recall': 0.8405565131740951,
 'support': 8122}
weighted
{'f1-score': 0.8468879413092608,
 'precision': 0.8673867385913735,
 'recall': 0.8405565131740951,
 'support': 8122}
## test
PPCR: 0.947115
### report
macro
{'f1-score': 0.6773697650538757,
 'precision': 0.7052364749155704,
 'recall': 0.6553672202112031,
 'support': 1970}
micro
{'f1-score': 0.9035532994923858,
 'precision': 0.9035532994923858,
 'recall': 0.9035532994923858,
 'support': 1970}
weighted
{'f1-score': 0.8981635754688349,
 'precision': 0.8974553542236673,
 'recall': 0.9035532994923858,
 'support': 1970}
### report_full
macro
{'f1-score': 0.6461564181022621,
 'precision': 0.7052364749155704,
 'recall': 0.6092338628202887,
 'support': 2080}
micro
{'f1-score': 0.8790123456790124,
 'precision': 0.9035532994923858,
 'recall': 0.8557692307692307,
 'support': 2080}
weighted
{'f1-score': 0.8663355729174597,
 'precision': 0.8911599650601337,
 'recall': 0.8557692307692307,
 'support': 2080}
```

## javascript
### Summary
29 rules, avg.len. 3.8

| | |
|-|-|
|Min support|140|
|Max support|1170|
|Min confidence|0.9208333492279053|
|Max confidence|0.996835470199585|

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
                     'base_model_name': 'sklearn.ensemble.RandomForestClassifier',
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
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 1154.` |
| 2 | `  -1.reserved = ,<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.921. Support: 360.` |
| 3 | `  -1.reserved = {<br>	∧ +3.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.936. Support: 228.` |
| 4 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 1169.` |
| 5 | `  -1.internal_type = Identifier<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 154.` |
| 6 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = ,<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.926. Support: 373.` |
| 7 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = {<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.943. Support: 256.` |
| 8 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = :<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 547.` |
| 9 | `  -1.roles in {IDENTIFIER}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.949. Support: 1037.` |
| 10 | `  -1.reserved = {<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.945. Support: 246.` |
| 11 | `  -1.reserved = :<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 520.` |
| 12 | `  -1.reserved = ,<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.925. Support: 342.` |
| 13 | `  -1.reserved not in {,}<br>	∧ -2.internal_type = Identifier<br>	∧ +1.reserved not in {}}<br>	∧ +3.roles in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 599.` |
| 14 | `  -1.reserved = {<br>	∧ +3.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.938. Support: 168.` |
| 15 | `  -1.internal_type = Identifier<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles in {MAP} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 840.` |
| 16 | `  -1.internal_type = Identifier<br>	∧ +1.reserved not in {=}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {MAP, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 183.` |
| 17 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {,, {}<br>	∧ -2.length ≥ 2<br>	∧ +1.reserved not in {}}<br>	∧ +3.roles in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.933. Support: 786.` |
| 18 | `  -1.internal_type = Identifier<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 144.` |
| 19 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = {<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.927. Support: 255.` |
| 20 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = :<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 520.` |
| 21 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {,, :, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {OPERATOR} and not in {DECLARATION, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.930. Support: 194.` |
| 22 | `  -1.reserved = return<br>	∧ -2.internal_type not in {Identifier}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +3.roles in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 158.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, {}<br>	∧ -2.roles in {EXPRESSION}<br>	∧ +3.roles not in {EXPRESSION}<br>	∧ ^1.roles in {MAP} and not in {QUALIFIED}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.939. Support: 140.` |
| 24 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.995. Support: 1170.` |
| 25 | `  -1.internal_type = Identifier<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.945. Support: 1008.` |
| 26 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = {<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.930. Support: 293.` |
| 27 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = :<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.987. Support: 514.` |
| 28 | `  -1.reserved not in {,}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +3.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.968. Support: 554.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +3.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.921. Support: 146.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 3.8275862068965516, "max_conf": 0.996835470199585, "max_support": 1170, "min_conf": 0.9208333492279053, "min_support": 140, "num_rules": 29}}
```
</details>
