# Model report for file:///tmp/top-repos-quality-repos-xlc6v3s5/open-decision.git HEAD d8ebd55ed79157c881455527501b1389d8f41f45

### Dump

```json
{'created_at': '2021-08-24 12:15:35',
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
 'size': '13.4 kB',
 'tags': [],
 'uuid': 'a70a86b6-565f-4ebc-9800-35c36ce76d61',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-xlc6v3s5/open-decision.git d8ebd55ed79157c881455527501b1389d8f41f45

# javascript
7 rules, avg.len. 3.9
## train
PPCR: 0.769183
### report
macro
{'f1-score': 0.8518738725692819,
 'precision': 0.9174172843078179,
 'recall': 0.8094263452198619,
 'support': 2456}
micro
{'f1-score': 0.8978013029315961,
 'precision': 0.8978013029315961,
 'recall': 0.8978013029315961,
 'support': 2456}
weighted
{'f1-score': 0.8908491695233609,
 'precision': 0.897467654408297,
 'recall': 0.8978013029315961,
 'support': 2456}
### report_full
macro
{'f1-score': 0.7203629677279791,
 'precision': 0.9174172843078179,
 'recall': 0.6368417755260616,
 'support': 3193}
micro
{'f1-score': 0.7806691449814127,
 'precision': 0.8978013029315961,
 'recall': 0.690573128719073,
 'support': 3193}
weighted
{'f1-score': 0.743191136601753,
 'precision': 0.8906851637573802,
 'recall': 0.690573128719073,
 'support': 3193}
## test
PPCR: 0.754197
### report
macro
{'f1-score': 0.7222102751666256,
 'precision': 0.9243065922348366,
 'recall': 0.6446648256494227,
 'support': 3145}
micro
{'f1-score': 0.859141494435612,
 'precision': 0.859141494435612,
 'recall': 0.859141494435612,
 'support': 3145}
weighted
{'f1-score': 0.8404617622951903,
 'precision': 0.8695207224225971,
 'recall': 0.859141494435612,
 'support': 3145}
### report_full
macro
{'f1-score': 0.5611141998210156,
 'precision': 0.9243065922348366,
 'recall': 0.4480292621841287,
 'support': 4170}
micro
{'f1-score': 0.738755980861244,
 'precision': 0.859141494435612,
 'recall': 0.6479616306954437,
 'support': 4170}
weighted
{'f1-score': 0.6987860835396791,
 'precision': 0.8692779561961671,
 'recall': 0.6479616306954437,
 'support': 4170}
```

## javascript
### Summary
4 rules, avg.len. 3.0

| | |
|-|-|
|Min support|119|
|Max support|494|
|Min confidence|0.9375|
|Max confidence|0.9961538314819336|

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
| 1 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 494.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.996. Support: 130.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.987. Support: 119.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.roles in {EXPRESSION, KEY}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.938. Support: 120.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 3.0, "max_conf": 0.9961538314819336, "max_support": 494, "min_conf": 0.9375, "min_support": 119, "num_rules": 4}}
```
</details>
