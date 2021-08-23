# Model report for file:///tmp/top-repos-quality-repos-2_e7t08_/phogo_v2.git HEAD 513f38b722e565568f7dd8b14c5c8db132e37b3a

### Dump

```json
{'created_at': '2021-08-21 05:54:33',
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
 'size': '15.7 kB',
 'tags': [],
 'uuid': '212f1f7f-679e-4987-a468-093bd84f5e20',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-2_e7t08_/phogo_v2.git 513f38b722e565568f7dd8b14c5c8db132e37b3a

# javascript
10 rules, avg.len. 5.0
## train
PPCR: 0.725468
### report
macro
{'f1-score': 0.40722273918953117,
 'precision': 0.4118552023437304,
 'recall': 0.4031858362033435,
 'support': 3874}
micro
{'f1-score': 0.9512132163138874,
 'precision': 0.9512132163138874,
 'recall': 0.9512132163138874,
 'support': 3874}
weighted
{'f1-score': 0.9426000876322246,
 'precision': 0.9351855430231325,
 'recall': 0.9512132163138874,
 'support': 3874}
### report_full
macro
{'f1-score': 0.38103811629560547,
 'precision': 0.4118552023437304,
 'recall': 0.35829603842376606,
 'support': 5340}
micro
{'f1-score': 0.7998697634035163,
 'precision': 0.9512132163138874,
 'recall': 0.6900749063670412,
 'support': 5340}
weighted
{'f1-score': 0.7215588596985028,
 'precision': 0.7666877302533779,
 'recall': 0.6900749063670412,
 'support': 5340}
## test
PPCR: 0.707631
### report
macro
{'f1-score': 0.4081263010945058,
 'precision': 0.40893563424051227,
 'recall': 0.4075560658621195,
 'support': 881}
micro
{'f1-score': 0.9557321225879682,
 'precision': 0.9557321225879682,
 'recall': 0.9557321225879682,
 'support': 881}
weighted
{'f1-score': 0.9478041132700523,
 'precision': 0.9405534382559521,
 'recall': 0.9557321225879682,
 'support': 881}
### report_full
macro
{'f1-score': 0.3833505272496532,
 'precision': 0.40893563424051227,
 'recall': 0.3640829908408271,
 'support': 1245}
micro
{'f1-score': 0.7920978363123237,
 'precision': 0.9557321225879682,
 'recall': 0.6763052208835342,
 'support': 1245}
weighted
{'f1-score': 0.705908379000369,
 'precision': 0.7446820637857958,
 'recall': 0.6763052208835342,
 'support': 1245}
```

## javascript
### Summary
9 rules, avg.len. 4.8

| | |
|-|-|
|Min support|114|
|Max support|1140|
|Min confidence|0.9253246784210205|
|Max confidence|0.9977973699569702|

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
                     'min_samples_split': 227,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.940. Support: 776.` |
| 2 | `  +1.reserved = }<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.965. Support: 128.` |
| 3 | `  -1.reserved = (<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 159.` |
| 4 | `  -1.reserved not in {(}<br>	∧ -2.label in {<space>}<br>	∧ -3.internal_type = Identifier<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 227.` |
| 5 | `  -1.reserved not in {(}<br>	∧ -2.label in {<newline>} and not in {<space>}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.925. Support: 154.` |
| 6 | `  +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 158.` |
| 7 | `  -1.roles not in {STRING}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 114.` |
| 8 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.931. Support: 124.` |
| 9 | `  -1.roles not in {STRING}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.930. Support: 1140.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.777777777777778, "max_conf": 0.9977973699569702, "max_support": 1140, "min_conf": 0.9253246784210205, "min_support": 114, "num_rules": 9}}
```
</details>
