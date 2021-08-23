# Model report for file:///tmp/top-repos-quality-repos-cu7e4e1x/diy-biller.git HEAD 7257ab7c67bd8e36e2da9a99f66c98dfaad6ab37

### Dump

```json
{'created_at': '2021-08-22 02:31:24',
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
 'size': '15.9 kB',
 'tags': [],
 'uuid': '9f3f2156-7df7-4503-8c76-11964c49dceb',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-cu7e4e1x/diy-biller.git 7257ab7c67bd8e36e2da9a99f66c98dfaad6ab37

# javascript
7 rules, avg.len. 4.9
## train
PPCR: 0.704282
### report
macro
{'f1-score': 0.6461375878961723,
 'precision': 0.6824538988021611,
 'recall': 0.6225151949937314,
 'support': 5461}
micro
{'f1-score': 0.9289507416224134,
 'precision': 0.9289507416224134,
 'recall': 0.9289507416224134,
 'support': 5461}
weighted
{'f1-score': 0.9208522985019633,
 'precision': 0.9223172182265862,
 'recall': 0.9289507416224134,
 'support': 5461}
### report_full
macro
{'f1-score': 0.48721316379495205,
 'precision': 0.6824538988021611,
 'recall': 0.411704241587135,
 'support': 7754}
micro
{'f1-score': 0.7677639046538026,
 'precision': 0.9289507416224134,
 'recall': 0.6542429713696157,
 'support': 7754}
weighted
{'f1-score': 0.7269587173228135,
 'precision': 0.9030998905920539,
 'recall': 0.6542429713696157,
 'support': 7754}
## test
PPCR: 0.849128
### report
macro
{'f1-score': 0.571587986160652,
 'precision': 0.669441743112392,
 'recall': 0.5424488884982777,
 'support': 1801}
micro
{'f1-score': 0.8945030538589671,
 'precision': 0.8945030538589672,
 'recall': 0.8945030538589672,
 'support': 1801}
weighted
{'f1-score': 0.8757614258620051,
 'precision': 0.8776914338174974,
 'recall': 0.8945030538589672,
 'support': 1801}
### report_full
macro
{'f1-score': 0.4597235464142395,
 'precision': 0.669441743112392,
 'recall': 0.4064892205799646,
 'support': 2121}
micro
{'f1-score': 0.8215196328403875,
 'precision': 0.8945030538589672,
 'recall': 0.7595473833097596,
 'support': 2121}
weighted
{'f1-score': 0.7833395816967339,
 'precision': 0.8683417033453023,
 'recall': 0.7595473833097596,
 'support': 2121}
```

## javascript
### Summary
6 rules, avg.len. 4.7

| | |
|-|-|
|Min support|92|
|Max support|3300|
|Min confidence|0.9213636517524719|
|Max confidence|0.9975961446762085|

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
                     'min_samples_split': 240,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = StringLiteral<br>	∧ -3.internal_type = JSXIdentifier<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = "<br>Confidence: 0.998. Support: 208.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ -3.internal_type not in {JSXIdentifier}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.935. Support: 146.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type = JSXAttribute<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = "<br>Confidence: 0.998. Support: 202.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = JSXOpeningElement<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 229.` |
| 5 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {;}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.984. Support: 92.` |
| 6 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {DECLARATION, MODULE, OPERATOR}<br>	∧ ^2.roles not in {FUNCTION}<br>⇒ y = ∅<br>Confidence: 0.921. Support: 3300.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.666666666666667, "max_conf": 0.9975961446762085, "max_support": 3300, "min_conf": 0.9213636517524719, "min_support": 92, "num_rules": 6}}
```
</details>
