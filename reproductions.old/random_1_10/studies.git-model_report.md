# Model report for file:///tmp/top-repos-quality-repos-ks4hzi1_/studies.git HEAD cdb8206b5bd94bbbb5a310e0bb00302f988e180c

### Dump

```json
{'created_at': '2021-08-20 12:36:26',
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
 'size': '18.4 kB',
 'tags': [],
 'uuid': '59765216-c079-415f-bbd2-61fe4397415d',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ks4hzi1_/studies.git cdb8206b5bd94bbbb5a310e0bb00302f988e180c

# javascript
11 rules, avg.len. 6.1
## train
PPCR: 0.698778
### report
macro
{'f1-score': 0.26434594798256195,
 'precision': 0.2756821780904376,
 'recall': 0.25605739014374307,
 'support': 11040}
micro
{'f1-score': 0.9354166666666666,
 'precision': 0.9354166666666667,
 'recall': 0.9354166666666667,
 'support': 11040}
weighted
{'f1-score': 0.9239276037840848,
 'precision': 0.9150388720112461,
 'recall': 0.9354166666666667,
 'support': 11040}
### report_full
macro
{'f1-score': 0.21488983304829484,
 'precision': 0.2756821780904376,
 'recall': 0.18674766845950636,
 'support': 15799}
micro
{'f1-score': 0.7695517716755468,
 'precision': 0.9354166666666667,
 'recall': 0.653648965124375,
 'support': 15799}
weighted
{'f1-score': 0.69662145825281,
 'precision': 0.7698927779992402,
 'recall': 0.653648965124375,
 'support': 15799}
## test
PPCR: 0.708152
### report
macro
{'f1-score': 0.21677992029960008,
 'precision': 0.25780693470663074,
 'recall': 0.20794053611998814,
 'support': 2936}
micro
{'f1-score': 0.9356267029972752,
 'precision': 0.9356267029972752,
 'recall': 0.9356267029972752,
 'support': 2936}
weighted
{'f1-score': 0.9213367074476659,
 'precision': 0.9172945748502923,
 'recall': 0.9356267029972752,
 'support': 2936}
### report_full
macro
{'f1-score': 0.18262515879220825,
 'precision': 0.25780693470663074,
 'recall': 0.16635828470432576,
 'support': 4146}
micro
{'f1-score': 0.7757695566224231,
 'precision': 0.9356267029972752,
 'recall': 0.6625663289917993,
 'support': 4146}
weighted
{'f1-score': 0.6890973485880725,
 'precision': 0.7555119513230528,
 'recall': 0.6625663289917993,
 'support': 4146}
```

## javascript
### Summary
8 rules, avg.len. 5.0

| | |
|-|-|
|Min support|141|
|Max support|2932|
|Min confidence|0.9276729822158813|
|Max confidence|0.9994344115257263|

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
                     'min_samples_split': 188,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.roles in {IDENTIFIER}<br>	∧ ^1.roles in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 171.` |
| 2 | `  -1.roles in {IDENTIFIER}<br>	∧ ^1.roles in {OPERATOR} and not in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.968. Support: 141.` |
| 3 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.length ≤ 2<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 2932.` |
| 4 | `  -1.reserved = .<br>	∧ -1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 884.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 447.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {.}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 188.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {., ;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.928. Support: 159.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {., ;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {(, =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 246.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.0, "max_conf": 0.9994344115257263, "max_support": 2932, "min_conf": 0.9276729822158813, "min_support": 141, "num_rules": 8}}
```
</details>
