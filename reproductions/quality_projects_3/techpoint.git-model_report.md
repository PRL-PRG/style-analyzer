# Model report for file:///tmp/top-repos-quality-repos-f55ya_6v/techpoint.git HEAD 911184c8b87b58843db9b96507f59d6a236da953

### Dump

```json
{'created_at': '2021-08-29 10:25:06',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.19.0-12-amd64-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '15.8 kB',
 'tags': [],
 'uuid': 'a905d561-5d6d-4042-898d-5bd113dbfb48',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-f55ya_6v/techpoint.git 911184c8b87b58843db9b96507f59d6a236da953

# javascript
13 rules, avg.len. 5.2
## train
PPCR: 0.762910
### report
macro
{'f1-score': 0.5001230466534367,
 'precision': 0.5086872809590676,
 'recall': 0.4932503449489746,
 'support': 7771}
micro
{'f1-score': 0.9085059837858706,
 'precision': 0.9085059837858706,
 'recall': 0.9085059837858706,
 'support': 7771}
weighted
{'f1-score': 0.8992554556963912,
 'precision': 0.8912690678634894,
 'recall': 0.9085059837858706,
 'support': 7771}
### report_full
macro
{'f1-score': 0.43693736631527297,
 'precision': 0.5086872809590676,
 'recall': 0.39655741154851476,
 'support': 10186}
micro
{'f1-score': 0.7863228824413877,
 'precision': 0.9085059837858706,
 'recall': 0.6931081877086197,
 'support': 10186}
weighted
{'f1-score': 0.7453398433112812,
 'precision': 0.8197471173113441,
 'recall': 0.6931081877086197,
 'support': 10186}
## test
PPCR: 0.748837
### report
macro
{'f1-score': 0.5106539198285028,
 'precision': 0.5092387164766612,
 'recall': 0.5126670910200001,
 'support': 1932}
micro
{'f1-score': 0.9218426501035196,
 'precision': 0.9218426501035196,
 'recall': 0.9218426501035196,
 'support': 1932}
weighted
{'f1-score': 0.9176244043735373,
 'precision': 0.914546403634852,
 'recall': 0.9218426501035196,
 'support': 1932}
### report_full
macro
{'f1-score': 0.4404969931996094,
 'precision': 0.5092387164766612,
 'recall': 0.398823347843212,
 'support': 2580}
micro
{'f1-score': 0.7894503546099291,
 'precision': 0.9218426501035196,
 'recall': 0.6903100775193799,
 'support': 2580}
weighted
{'f1-score': 0.7520427611108881,
 'precision': 0.8387550599352087,
 'recall': 0.6903100775193799,
 'support': 2580}
```

## javascript
### Summary
7 rules, avg.len. 5.1

| | |
|-|-|
|Min support|100|
|Max support|416|
|Min confidence|0.9806867241859436|
|Max confidence|0.9987980723381042|

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
                     'min_samples_split': 182,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.999. Support: 351.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.995. Support: 106.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.reserved = ><br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 125.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved = =<br>	∧ +2.roles in {IDENTIFIER}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 100.` |
| 5 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {IDENTIFIER} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 416.` |
| 6 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 334.` |
| 7 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 233.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.142857142857143, "max_conf": 0.9987980723381042, "max_support": 416, "min_conf": 0.9806867241859436, "min_support": 100, "num_rules": 7}}
```
</details>
