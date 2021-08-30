# Model report for file:///tmp/top-repos-quality-repos-h5mm5sj6/toscani.git HEAD 98bd486591029911cfe1a0aba946e25bef0fcc44

### Dump

```json
{'created_at': '2021-08-30 04:35:48',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.11.0-31-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '15.8 kB',
 'tags': [],
 'uuid': '45d8aa8f-510b-4bb2-83d9-d6e3fa0880ff',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-h5mm5sj6/toscani.git 98bd486591029911cfe1a0aba946e25bef0fcc44

# javascript
11 rules, avg.len. 5.0
## train
PPCR: 0.901283
### report
macro
{'f1-score': 0.5084037161653848,
 'precision': 0.5067177793805723,
 'recall': 0.5113929200068807,
 'support': 9623}
micro
{'f1-score': 0.9066819079289203,
 'precision': 0.9066819079289203,
 'recall': 0.9066819079289203,
 'support': 9623}
weighted
{'f1-score': 0.8885195509300523,
 'precision': 0.8731189630671018,
 'recall': 0.9066819079289203,
 'support': 9623}
### report_full
macro
{'f1-score': 0.50075642261124,
 'precision': 0.5067177793805723,
 'recall': 0.496620152312457,
 'support': 10677}
micro
{'f1-score': 0.8596059113300493,
 'precision': 0.9066819079289203,
 'recall': 0.8171771096750023,
 'support': 10677}
weighted
{'f1-score': 0.8094912560110251,
 'precision': 0.803800083840478,
 'recall': 0.8171771096750023,
 'support': 10677}
## test
PPCR: 0.882249
### report
macro
{'f1-score': 0.2531264128352094,
 'precision': 0.23650653375050684,
 'recall': 0.2746281567273039,
 'support': 1993}
micro
{'f1-score': 0.8625188158554942,
 'precision': 0.8625188158554943,
 'recall': 0.8625188158554943,
 'support': 1993}
weighted
{'f1-score': 0.8189551700839448,
 'precision': 0.784100225128286,
 'recall': 0.8625188158554943,
 'support': 1993}
### report_full
macro
{'f1-score': 0.25146313349753147,
 'precision': 0.23650653375050684,
 'recall': 0.27046090586332155,
 'support': 2259}
micro
{'f1-score': 0.8085606773283162,
 'precision': 0.8625188158554943,
 'recall': 0.7609561752988048,
 'support': 2259}
weighted
{'f1-score': 0.7268986893740275,
 'precision': 0.6993141359524674,
 'recall': 0.7609561752988048,
 'support': 2259}
```

## javascript
### Summary
9 rules, avg.len. 4.4

| | |
|-|-|
|Min support|100|
|Max support|1581|
|Min confidence|0.936956524848938|
|Max confidence|0.9991708397865295|

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
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 1377.` |
| 2 | `  -1.roles in {EXPRESSION}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.942. Support: 334.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 141.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 1581.` |
| 5 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 603.` |
| 6 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 284.` |
| 7 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.937. Support: 230.` |
| 8 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles in {COMMENT}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 104.` |
| 9 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>⇒ y = ∅<br>Confidence: 0.975. Support: 100.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.444444444444445, "max_conf": 0.9991708397865295, "max_support": 1581, "min_conf": 0.936956524848938, "min_support": 100, "num_rules": 9}}
```
</details>
