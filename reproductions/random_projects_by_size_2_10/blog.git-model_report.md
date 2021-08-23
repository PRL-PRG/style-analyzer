# Model report for file:///tmp/top-repos-quality-repos-9642l4ut/blog.git HEAD 3004d96740f06a27bebf8b9c492db7babea45aa3

### Dump

```json
{'created_at': '2021-08-22 04:42:31',
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
 'size': '15.0 kB',
 'tags': [],
 'uuid': '223d7a46-1fd9-4ef8-b922-d2175268c352',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-9642l4ut/blog.git 3004d96740f06a27bebf8b9c492db7babea45aa3

# javascript
12 rules, avg.len. 5.2
## train
PPCR: 0.896153
### report
macro
{'f1-score': 0.9055611670634135,
 'precision': 0.9471821230951974,
 'recall': 0.8773209652794596,
 'support': 6826}
micro
{'f1-score': 0.9371520656314093,
 'precision': 0.9371520656314093,
 'recall': 0.9371520656314093,
 'support': 6826}
weighted
{'f1-score': 0.9360107665849742,
 'precision': 0.9416916854075581,
 'recall': 0.9371520656314093,
 'support': 6826}
### report_full
macro
{'f1-score': 0.8525340910134966,
 'precision': 0.9471821230951974,
 'recall': 0.789281894377443,
 'support': 7617}
micro
{'f1-score': 0.8858270442428857,
 'precision': 0.9371520656314093,
 'recall': 0.8398319548378627,
 'support': 7617}
weighted
{'f1-score': 0.8825507910426664,
 'precision': 0.938583118976715,
 'recall': 0.8398319548378627,
 'support': 7617}
## test
PPCR: 0.912000
### report
macro
{'f1-score': 0.6022292384232683,
 'precision': 0.5277777777777778,
 'recall': 0.8057820607857673,
 'support': 114}
micro
{'f1-score': 0.9122807017543859,
 'precision': 0.9122807017543859,
 'recall': 0.9122807017543859,
 'support': 114}
weighted
{'f1-score': 0.9218255886559107,
 'precision': 0.9444444444444443,
 'recall': 0.9122807017543859,
 'support': 114}
### report_full
macro
{'f1-score': 0.55609113403387,
 'precision': 0.5277777777777778,
 'recall': 0.6995238095238095,
 'support': 125}
micro
{'f1-score': 0.8702928870292886,
 'precision': 0.9122807017543859,
 'recall': 0.832,
 'support': 125}
weighted
{'f1-score': 0.8770322953822423,
 'precision': 0.9365333333333333,
 'recall': 0.832,
 'support': 125}
```

## javascript
### Summary
9 rules, avg.len. 4.8

| | |
|-|-|
|Min support|122|
|Max support|1278|
|Min confidence|0.9351415038108826|
|Max confidence|0.998046875|

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
| 1 | `  +1.roles not in {STRING}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 424.` |
| 2 | `  +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.984. Support: 282.` |
| 3 | `  -1.reserved = {<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.994. Support: 248.` |
| 4 | `  -1.reserved not in {{}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {IDENTIFIER} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 1278.` |
| 5 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = '<br>Confidence: 0.996. Support: 139.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 807.` |
| 7 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = '<br>Confidence: 0.996. Support: 122.` |
| 8 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 415.` |
| 9 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 256.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.777777777777778, "max_conf": 0.998046875, "max_support": 1278, "min_conf": 0.9351415038108826, "min_support": 122, "num_rules": 9}}
```
</details>
