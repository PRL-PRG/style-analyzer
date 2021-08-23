# Model report for file:///tmp/top-repos-quality-repos-okbqn8k2/native-app.git HEAD 5f7d469f44aa7b236a80d66641a505d5f95a0b6c

### Dump

```json
{'created_at': '2021-08-21 05:16:19',
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
 'size': '17.4 kB',
 'tags': [],
 'uuid': 'f2538107-314d-417e-ae89-a56b8622adc6',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-okbqn8k2/native-app.git 5f7d469f44aa7b236a80d66641a505d5f95a0b6c

# javascript
21 rules, avg.len. 7.9
## train
PPCR: 0.763324
### report
macro
{'f1-score': 0.7855273986635977,
 'precision': 0.8083179984553286,
 'recall': 0.7677031212327936,
 'support': 14523}
micro
{'f1-score': 0.9570336707291882,
 'precision': 0.9570336707291882,
 'recall': 0.9570336707291882,
 'support': 14523}
weighted
{'f1-score': 0.9558552363801637,
 'precision': 0.9562108432845686,
 'recall': 0.9570336707291882,
 'support': 14523}
### report_full
macro
{'f1-score': 0.5991541768234676,
 'precision': 0.8083179984553286,
 'recall': 0.493993217804232,
 'support': 19026}
micro
{'f1-score': 0.8285790932665654,
 'precision': 0.9570336707291882,
 'recall': 0.7305266477451908,
 'support': 19026}
weighted
{'f1-score': 0.8081206279638161,
 'precision': 0.9366070900513068,
 'recall': 0.7305266477451908,
 'support': 19026}
## test
PPCR: 0.739578
### report
macro
{'f1-score': 0.7769907655483463,
 'precision': 0.8173052068649773,
 'recall': 0.7484890918323406,
 'support': 2945}
micro
{'f1-score': 0.9463497453310696,
 'precision': 0.9463497453310696,
 'recall': 0.9463497453310696,
 'support': 2945}
weighted
{'f1-score': 0.9429801148220052,
 'precision': 0.9430168218529509,
 'recall': 0.9463497453310696,
 'support': 2945}
### report_full
macro
{'f1-score': 0.5833946790539395,
 'precision': 0.8173052068649773,
 'recall': 0.47952473074402224,
 'support': 3982}
micro
{'f1-score': 0.8046773495019489,
 'precision': 0.9463497453310696,
 'recall': 0.6998995479658463,
 'support': 3982}
weighted
{'f1-score': 0.7816239757682146,
 'precision': 0.92780315204515,
 'recall': 0.6998995479658463,
 'support': 3982}
```

## javascript
### Summary
12 rules, avg.len. 8.5

| | |
|-|-|
|Min support|90|
|Max support|7090|
|Min confidence|0.9261168241500854|
|Max confidence|0.9982638955116272|

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
| 1 | `  -1.label in {<space>}<br>	∧ +1.roles in {LITERAL}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = "<br>Confidence: 0.985. Support: 98.` |
| 2 | `  •••start_col ≥ 65<br>	∧ -1.label in {<space>}<br>	∧ +1.roles in {LITERAL}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = '<br>Confidence: 0.972. Support: 90.` |
| 3 | `  •••start_col ≤ 36<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.roles in {KEY} and not in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.981. Support: 398.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.933. Support: 112.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.926. Support: 291.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 172.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 145.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.roles in {KEY} and not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles in {MAP}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.977. Support: 108.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY, LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type = Program<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 92.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {KEY, LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {Program}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 288.` |
| 11 | `  -1.diff_col ≤ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -2.label not in {<newline>}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -3.reserved = :<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY, LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {Program}<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR}<br>	∧ ^2.roles not in {SCOPE}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 233.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -2.label not in {<newline>}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -3.reserved not in {:}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY, LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {Program}<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR}<br>	∧ ^2.roles not in {SCOPE}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 7090.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.5, "max_conf": 0.9982638955116272, "max_support": 7090, "min_conf": 0.9261168241500854, "min_support": 90, "num_rules": 12}}
```
</details>
